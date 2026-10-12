use std::ops::Range;
use std::path::Path;

use anyhow::{Context, Result};
use goblin::pe::PE;
use goblin::pe::section_table::IMAGE_SCN_MEM_EXECUTE;

use crate::pattern::Pattern;
use crate::rva::Rva;
use crate::symbol_store::PdbId;

/// A DLL read from disk, addressed by RVA just like when the game has it loaded.
pub struct DllImage {
    bytes: Vec<u8>,
    /// The address the DLL prefers to be loaded at, which absolute pointers in it assume.
    image_base: u64,
    sections: Vec<Section>,
    /// Function bounds from the exception table (`.pdata`), sorted by start address.
    functions: Vec<Range<Rva>>,
    /// The PDB with this DLL's symbols, if the DLL records one.
    pdb: Option<PdbId>,
}

/// Where a section is loaded in memory and where its bytes are stored in the file.
struct Section {
    rvas: Range<Rva>,
    file_offset: usize,
    executable: bool,
}

impl DllImage {
    pub fn load(path: &Path) -> Result<Self> {
        let bytes = std::fs::read(path).with_context(|| format!("reading {}", path.display()))?;
        let pe = PE::parse(&bytes).with_context(|| format!("parsing {}", path.display()))?;

        let sections = pe
            .sections
            .iter()
            .map(|section| {
                let start = Rva(section.virtual_address);
                // The file stores the section padded to the file alignment, so
                // `virtual_size` is what actually belongs to the section.
                let len = section.virtual_size.min(section.size_of_raw_data);
                Section {
                    rvas: start..start + len,
                    file_offset: section.pointer_to_raw_data as usize,
                    executable: section.characteristics & IMAGE_SCN_MEM_EXECUTE != 0,
                }
            })
            .collect();

        let mut functions = match &pe.exception_data {
            Some(exception_data) => exception_data
                .functions()
                .map(|function| function.map(|f| Rva(f.begin_address)..Rva(f.end_address)))
                .collect::<Result<Vec<_>, _>>()?,
            None => Vec::new(),
        };
        functions.sort_by_key(|function| function.start);

        let pdb = pe
            .debug_data
            .as_ref()
            .and_then(|debug_data| debug_data.codeview_pdb70_debug_info.as_ref())
            .and_then(PdbId::from_codeview);

        Ok(Self {
            image_base: pe.image_base,
            bytes,
            sections,
            functions,
            pdb,
        })
    }

    pub fn image_base(&self) -> u64 {
        self.image_base
    }

    pub fn pdb(&self) -> Option<&PdbId> {
        self.pdb.as_ref()
    }

    /// The `len` bytes at `rva`, if they're all inside one section.
    pub fn bytes_at(&self, rva: Rva, len: usize) -> Option<&[u8]> {
        let section = self.sections.iter().find(|s| s.rvas.contains(&rva))?;
        let offset = (rva - section.rvas.start) as usize;
        self.section_bytes(section).get(offset..offset + len)
    }

    /// Every match of `pattern` in the executable sections.
    pub fn find_code<'a>(&'a self, pattern: &'a Pattern) -> impl Iterator<Item = Rva> + 'a {
        self.code_sections().flat_map(move |(start, code)| {
            pattern
                .find_in(code)
                .map(move |offset| start + offset as u32)
        })
    }

    /// Every match of `pattern` in any section, code or data.
    pub fn find_anywhere<'a>(&'a self, pattern: &'a Pattern) -> impl Iterator<Item = Rva> + 'a {
        self.all_sections().flat_map(move |(start, bytes)| {
            pattern
                .find_in(bytes)
                .map(move |offset| start + offset as u32)
        })
    }

    /// The bytes of each executable section, with the RVA they start at.
    pub fn code_sections(&self) -> impl Iterator<Item = (Rva, &[u8])> {
        self.all_sections_where(|section| section.executable)
    }

    /// The bytes of every section, with the RVA they start at.
    pub fn all_sections(&self) -> impl Iterator<Item = (Rva, &[u8])> {
        self.all_sections_where(|_| true)
    }

    fn all_sections_where(
        &self,
        include: impl Fn(&Section) -> bool,
    ) -> impl Iterator<Item = (Rva, &[u8])> {
        self.sections
            .iter()
            .filter(move |section| include(section))
            .map(|section| (section.rvas.start, self.section_bytes(section)))
    }

    /// The bounds of the function that contains `rva`, according to the exception table.
    ///
    /// The compiler sometimes splits one function into several entries, so this can
    /// be a part of a function rather than all of it. It always starts on an
    /// instruction, which is what disassembly needs.
    pub fn function_at(&self, rva: Rva) -> Option<Range<Rva>> {
        let after = self.functions.partition_point(|f| f.start <= rva);
        let function = self.functions.get(after.checked_sub(1)?)?;
        function.contains(&rva).then(|| function.clone())
    }

    fn section_bytes(&self, section: &Section) -> &[u8] {
        let len = (section.rvas.end - section.rvas.start) as usize;
        &self.bytes[section.file_offset..section.file_offset + len]
    }
}
