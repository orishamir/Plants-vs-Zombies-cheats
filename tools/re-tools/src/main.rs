//! Reverse-engineering helpers for keeping the cheats' patches up to date when the game
//! updates. See `README.md` for the workflow.

mod disasm;
mod hex;
mod image;
mod pattern;
mod rva;
mod snapshot;
mod symbol_store;
mod symbols;
mod xrefs;

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail, ensure};
use clap::{Parser, Subcommand};
use pvz_sdk::{Dll, Location, entities, toggleables};

use hex::HexBytes;
use image::DllImage;
use pattern::Pattern;
use rva::Rva;
use symbols::Symbols;
use xrefs::Reference;

const DEFAULT_DLL: &str =
    r"C:\Program Files (x86)\Steam\steamapps\common\PVZ Replanted\GameAssembly.dll";

/// Find and inspect the code the cheats patch in the game's DLLs.
#[derive(Parser)]
struct Cli {
    /// The DLL to inspect: GameAssembly.dll for the game's code, UnityPlayer.dll for the engine.
    #[arg(long, default_value = DEFAULT_DLL)]
    dll: PathBuf,

    /// Il2CppDumper's output directory, which names GameAssembly.dll's methods (see README.md).
    #[arg(long, default_value = "il2cpp-dump")]
    dump: PathBuf,

    /// Where PDBs are kept, which name the engine's functions. `snapshot` downloads them.
    #[arg(long, default_value = "symbols")]
    symbols: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Check that every cheat's patches still match the game's code, and that the globals the
    /// entities are read from are still where the IL2CPP dump says.
    Check,
    /// Copy the game's files into `<to>/<Steam build ID>/`, to compare with after an update,
    /// and download the engine's PDB.
    Snapshot {
        #[arg(long, default_value = "game-versions")]
        to: PathBuf,
    },
    /// Find a byte pattern in the game's code, e.g. "83 AB 8C000000 04" or "7E ??".
    Find {
        pattern: Pattern,
        /// Only show matches at or after this address.
        #[arg(long)]
        from: Option<Rva>,
        /// Only show matches before this address.
        #[arg(long)]
        to: Option<Rva>,
    },
    /// Find a text string anywhere in the DLL, e.g. the name of an engine function.
    Text { text: String },
    /// Disassemble the code around an address.
    Disasm {
        rva: Rva,
        /// How many bytes before the address to show.
        #[arg(long, default_value_t = 64)]
        before: u32,
        /// How many bytes after the address to show.
        #[arg(long, default_value_t = 32)]
        after: u32,
    },
    /// List the `call`s and `jmp`s that go straight to an address, e.g. a function's start.
    Callers { rva: Rva },
    /// List the code and pointers that refer to an address, e.g. a string or a global.
    Refs { rva: Rva },
    /// Print the bytes at an address.
    Bytes {
        rva: Rva,
        #[arg(default_value_t = 16)]
        len: usize,
    },
    /// Show which named function contains each address.
    Where {
        #[arg(required = true)]
        rvas: Vec<Rva>,
    },
    /// List the names that contain some text, e.g. "Coin$$" or "RunInBackground".
    Names { query: String },
}

impl Cli {
    fn image(&self) -> Result<DllImage> {
        DllImage::load(&self.dll)
    }

    /// Names for `--dll`'s addresses. Most commands work without them and just show addresses.
    fn symbols(&self, image: &DllImage) -> Result<Option<Symbols>> {
        self.symbols_for(&self.dll, image)
    }

    /// Names for a DLL's addresses: the game's methods from the IL2CPP dump for
    /// GameAssembly.dll, or the DLL's PDB if the symbol store has it.
    fn symbols_for(&self, dll_path: &Path, image: &DllImage) -> Result<Option<Symbols>> {
        let is_game_assembly = dll_path
            .file_name()
            .is_some_and(|name| name.eq_ignore_ascii_case(Dll::GameAssembly.file_name()));
        if is_game_assembly {
            let has_dump = self.dump.join("script.json").exists();
            return has_dump
                .then(|| Symbols::from_il2cpp_dump(&self.dump))
                .transpose();
        }
        let Some(pdb) = image.pdb() else {
            return Ok(None);
        };
        let path = pdb.path_in(&self.symbols);
        path.exists().then(|| Symbols::from_pdb(&path)).transpose()
    }

    /// Like [`Cli::symbols`], for commands that are useless without names.
    fn required_symbols(&self, image: &DllImage) -> Result<Symbols> {
        self.symbols(image)?.context(
            "no names for this DLL: generate the IL2CPP dump for GameAssembly.dll, \
             or run `snapshot` to download the engine's PDB (see README.md)",
        )
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Check => check(&cli)?,
        Command::Snapshot { to } => {
            let game_dir = cli.dll.parent().context("--dll has no folder")?;
            let snapshot = snapshot::snapshot(game_dir, to)?;
            println!("The game's files are in {}", snapshot.display());
            download_engine_symbols(&snapshot.join(Dll::UnityPlayer.file_name()), &cli.symbols)?;
        }
        Command::Find { pattern, from, to } => {
            let image = cli.image()?;
            let symbols = cli.symbols(&image)?;
            let in_range =
                |rva: &Rva| from.is_none_or(|from| *rva >= from) && to.is_none_or(|to| *rva < to);

            let matches: Vec<Rva> = image.find_code(pattern).filter(in_range).collect();
            for &rva in &matches {
                println!("{rva:>10}  {}", describe(rva, symbols.as_ref()));
            }
            println!("{} matches", matches.len());
        }
        Command::Text { text } => {
            for rva in cli.image()?.find_anywhere(&Pattern::exact(text.as_bytes())) {
                println!("{rva:>10}");
            }
        }
        Command::Disasm { rva, before, after } => {
            let image = cli.image()?;
            let symbols = cli.symbols(&image)?;
            println!("{rva} is in {}", describe(*rva, symbols.as_ref()));
            disasm::print_around(&image, symbols.as_ref(), *rva, *before, *after)?;
        }
        Command::Callers { rva } => {
            let image = cli.image()?;
            let symbols = cli.symbols(&image)?;
            for branch in xrefs::branches_to(&image, *rva) {
                println!("{branch:>10}  {}", describe(branch, symbols.as_ref()));
            }
        }
        Command::Refs { rva } => {
            let image = cli.image()?;
            let symbols = cli.symbols(&image)?;
            for reference in xrefs::references_to(&image, *rva) {
                match reference {
                    Reference::RipRelative(at) => {
                        println!("{at:>10}  code     {}", describe(at, symbols.as_ref()));
                    }
                    Reference::Pointer(at) => println!("{at:>10}  pointer"),
                }
            }
        }
        Command::Bytes { rva, len } => {
            let image = cli.image()?;
            let bytes = image
                .bytes_at(*rva, *len)
                .with_context(|| format!("{len} bytes at {rva} aren't in the file"))?;
            println!("{rva}: {}", HexBytes(bytes));
        }
        Command::Where { rvas } => {
            let symbols = cli.required_symbols(&cli.image()?)?;
            for &rva in rvas {
                println!("{rva:>10}  {}", describe(rva, Some(&symbols)));
            }
        }
        Command::Names { query } => {
            for symbol in cli.required_symbols(&cli.image()?)?.matching(query) {
                println!("{:>10}  {}", symbol.start, symbol.name);
            }
        }
    }
    Ok(())
}

/// Downloads the PDB for the engine DLL at `unity_player` into the symbol store. Without it,
/// everything still works, just without the engine's names, so a failure only warns.
fn download_engine_symbols(unity_player: &Path, store: &Path) -> Result<()> {
    let image = DllImage::load(unity_player)?;
    let Some(pdb) = image.pdb() else {
        eprintln!("warning: {} doesn't name a PDB", unity_player.display());
        return Ok(());
    };
    match pdb.download_from_unity(store) {
        Ok(path) => println!("The engine's symbols are in {}", path.display()),
        Err(error) => eprintln!("warning: couldn't download the engine's symbols: {error:#}"),
    }
    Ok(())
}

/// Checks every address in the game's DLLs that the cheats and the entities use, printing
/// whether each still matches the game, and fails if any don't.
fn check(cli: &Cli) -> Result<()> {
    let mut dlls = GameDlls::new(cli);
    let broken = check_patches(&mut dlls)? + check_type_infos(&mut dlls)?;
    ensure!(
        broken == 0,
        "{broken} addresses don't match the game; README.md explains how to find them again"
    );
    Ok(())
}

/// Prints whether each cheat's patches find the game's original bytes where they expect them,
/// and returns how many don't.
fn check_patches(dlls: &mut GameDlls) -> Result<usize> {
    let mut broken = 0;
    for cheat in toggleables::ALL {
        for patch in cheat.patches {
            let location = patch.location.to_string();
            let Location::InDll { dll, offset } = patch.location else {
                println!(
                    "skip    {:<24} {location:<28}  only exists in the running game",
                    cheat.name
                );
                continue;
            };
            let (image, symbols) = dlls.get(dll)?;

            let rva = Rva(u32::try_from(offset)?);
            let found = image
                .bytes_at(rva, patch.original.len())
                .unwrap_or_default();
            let matches = found == patch.original;
            let status = if matches { "ok" } else { "BROKEN" };
            let name = describe(rva, symbols.as_ref());
            println!("{status:<6}  {:<24} {location:<28}  {name}", cheat.name);
            if !matches {
                broken += 1;
                println!(
                    "        expected {}, found {}",
                    HexBytes(patch.original),
                    HexBytes(found)
                );
            }
        }
    }
    Ok(broken)
}

/// Prints whether each `TypeInfo` global that reading the entities starts from is where
/// Il2CppDumper's `script.json` says it is, and returns how many aren't.
fn check_type_infos(dlls: &mut GameDlls) -> Result<usize> {
    let mut broken = 0;
    for type_info in entities::TYPE_INFOS {
        let location = type_info.location.to_string();
        let Location::InDll { dll, offset } = type_info.location else {
            bail!("{} isn't in a DLL", type_info.name);
        };
        let Some(symbols) = &dlls.get(dll)?.1 else {
            println!(
                "skip    {:<24} {location:<28}  needs the IL2CPP dump",
                "entities"
            );
            continue;
        };

        let found = symbols.address_of(type_info.name);
        let matches = found.is_some_and(|rva| usize::try_from(rva.0) == Ok(offset));
        let status = if matches { "ok" } else { "BROKEN" };
        println!(
            "{status:<6}  {:<24} {location:<28}  {}",
            "entities", type_info.name
        );
        if !matches {
            broken += 1;
            match found {
                Some(rva) => println!("        script.json has it at {rva}"),
                None => println!("        script.json doesn't have it"),
            }
        }
    }
    Ok(broken)
}

/// The game's DLLs, each read from the folder `--dll` is in the first time it's needed,
/// with names for their addresses if there are any.
struct GameDlls<'a> {
    cli: &'a Cli,
    loaded: HashMap<Dll, (DllImage, Option<Symbols>)>,
}

impl<'a> GameDlls<'a> {
    fn new(cli: &'a Cli) -> Self {
        Self {
            cli,
            loaded: HashMap::new(),
        }
    }

    fn get(&mut self, dll: Dll) -> Result<&(DllImage, Option<Symbols>)> {
        Ok(match self.loaded.entry(dll) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let path = self.cli.dll.with_file_name(dll.file_name());
                let image = DllImage::load(&path)?;
                let symbols = self.cli.symbols_for(&path, &image)?;
                entry.insert((image, symbols))
            }
        })
    }
}

/// Where `rva` is, as `name+offset`, or nothing without names.
fn describe(rva: Rva, symbols: Option<&Symbols>) -> String {
    match symbols.map(|symbols| symbols.containing(rva)) {
        Some(Some(symbol)) => format!("{}+{:#X}", symbol.name, rva - symbol.start),
        Some(None) => "no known name".to_owned(),
        None => String::new(),
    }
}
