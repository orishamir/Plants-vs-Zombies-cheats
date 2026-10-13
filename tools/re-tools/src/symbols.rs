//! Names for addresses in a DLL: the game's C# methods and IL2CPP's globals from Il2CppDumper,
//! or the engine's C++ functions and globals from its PDB.

use std::fs::File;
use std::path::Path;

use anyhow::{Context, Result};
use msvc_demangler::DemangleFlags;
use pdb::{FallibleIterator, SymbolData};
use serde::Deserialize;

use crate::rva::Rva;

/// Names and where they start, sorted by address.
pub struct Symbols(Vec<Symbol>);

pub struct Symbol {
    pub start: Rva,
    /// For the game's code, Il2CppDumper's `Class$$Method`, like `Plant$$UpdateChomper`, or the
    /// name of one of IL2CPP's globals, like `Reloaded.Gameplay.Board_TypeInfo`. For
    /// the engine, the C++ name, like `GetPlayerShouldRunInBackground`.
    pub name: String,
}

/// The parts of Il2CppDumper's `script.json` we use.
#[derive(Deserialize)]
struct Script {
    #[serde(rename = "ScriptMethod")]
    methods: Vec<ScriptEntry>,
    /// IL2CPP's globals, like the `<class>_TypeInfo`s that point to classes.
    #[serde(rename = "ScriptMetadata")]
    metadata: Vec<ScriptEntry>,
}

/// A named address.
#[derive(Deserialize)]
struct ScriptEntry {
    #[serde(rename = "Address")]
    address: u32,
    #[serde(rename = "Name")]
    name: String,
}

impl Symbols {
    /// The game's methods and IL2CPP's globals, from `script.json` in an Il2CppDumper output
    /// directory.
    pub fn from_il2cpp_dump(dump_dir: &Path) -> Result<Self> {
        let path = dump_dir.join("script.json");
        let json = std::fs::read_to_string(&path).with_context(|| {
            format!(
                "reading {} (see tools/re-tools/README.md to generate it)",
                path.display()
            )
        })?;
        let script: Script =
            serde_json::from_str(&json).with_context(|| format!("parsing {}", path.display()))?;

        let symbols = script
            .methods
            .into_iter()
            .chain(script.metadata)
            .filter(|entry| entry.address != 0)
            .map(|entry| Symbol {
                start: Rva(entry.address),
                name: entry.name,
            })
            .collect();
        Ok(Self::sorted(symbols))
    }

    /// The functions and globals in a PDB.
    pub fn from_pdb(path: &Path) -> Result<Self> {
        let file = File::open(path).with_context(|| format!("opening {}", path.display()))?;
        let mut pdb =
            pdb::PDB::open(file).with_context(|| format!("reading {}", path.display()))?;
        let address_map = pdb.address_map()?;
        let mut symbols = Vec::new();

        // Each compiled file's functions, with plain names. These include static functions,
        // which the public symbols below leave out. They come first so that when both name an
        // address, the plain name wins.
        let debug_information = pdb.debug_information()?;
        let mut modules = debug_information.modules()?;
        while let Some(module) = modules.next()? {
            let Some(module_info) = pdb.module_info(&module)? else {
                continue;
            };
            let mut module_symbols = module_info.symbols()?;
            while let Some(symbol) = module_symbols.next()? {
                if let Ok(SymbolData::Procedure(procedure)) = symbol.parse()
                    && let Some(rva) = procedure.offset.to_rva(&address_map)
                {
                    symbols.push(Symbol {
                        start: Rva(rva.0),
                        name: procedure.name.to_string().into_owned(),
                    });
                }
            }
        }

        // Every exported function and global, with C++ mangled names.
        let globals = pdb.global_symbols()?;
        let mut global_symbols = globals.iter();
        while let Some(symbol) = global_symbols.next()? {
            if let Ok(SymbolData::Public(public)) = symbol.parse()
                && let Some(rva) = public.offset.to_rva(&address_map)
            {
                symbols.push(Symbol {
                    start: Rva(rva.0),
                    name: demangle(&public.name.to_string()),
                });
            }
        }
        Ok(Self::sorted(symbols))
    }

    /// Sorts by address, keeping the first name given for each address.
    fn sorted(mut symbols: Vec<Symbol>) -> Self {
        symbols.sort_by_key(|symbol| symbol.start);
        symbols.dedup_by_key(|symbol| symbol.start);
        Self(symbols)
    }

    /// The symbol that contains `rva`: the closest one that starts at or before it.
    ///
    /// This is right for the game's methods, which IL2CPP places one after another. It can be
    /// wrong for code that has no name of its own, such as runtime helpers or the engine's
    /// functions the PDB doesn't list.
    pub fn containing(&self, rva: Rva) -> Option<&Symbol> {
        let after = self.0.partition_point(|symbol| symbol.start <= rva);
        self.0.get(after.checked_sub(1)?)
    }

    /// The symbol that starts exactly at `rva`, e.g. the target of a `call`.
    pub fn starting_at(&self, rva: Rva) -> Option<&Symbol> {
        self.containing(rva).filter(|symbol| symbol.start == rva)
    }

    /// Where the symbol named exactly `name` is.
    pub fn address_of(&self, name: &str) -> Option<Rva> {
        self.0
            .iter()
            .find(|symbol| symbol.name == name)
            .map(|symbol| symbol.start)
    }

    /// Symbols whose name contains `query`.
    pub fn matching<'a>(&'a self, query: &'a str) -> impl Iterator<Item = &'a Symbol> + 'a {
        self.0
            .iter()
            .filter(move |symbol| symbol.name.contains(query))
    }
}

/// `?SetPlayerPause@@YAXW4PlayerPause@@_N@Z` becomes `SetPlayerPause`. Names that aren't
/// mangled, or that the demangler can't read, stay as they are.
fn demangle(name: &str) -> String {
    msvc_demangler::demangle(name, DemangleFlags::NAME_ONLY).unwrap_or_else(|_| name.to_owned())
}
