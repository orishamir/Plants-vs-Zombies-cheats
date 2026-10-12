//! Finding the places that refer to an address ("cross-references").
//!
//! These scan bytes instead of decoding every instruction, so in rare cases a match is just
//! bytes that happen to look like a reference. `disasm` on a match shows whether it's real.

use crate::image::DllImage;
use crate::rva::Rva;

/// A place that refers to an address.
pub enum Reference {
    /// A 4-byte instruction operand relative to the end of the instruction, such as
    /// `lea rcx,[rip+0x1234]`. The RVA is where those 4 bytes are, inside the instruction.
    RipRelative(Rva),
    /// An absolute 8-byte pointer, such as an entry in a table of function pointers.
    Pointer(Rva),
}

/// Every `call` or `jmp` straight to `target`: opcode `E8` (call) or `E9` (jmp, used for
/// tail calls), followed by a 4-byte offset from the end of the instruction.
pub fn branches_to(image: &DllImage, target: Rva) -> impl Iterator<Item = Rva> + '_ {
    const BRANCH_LEN: usize = 5;

    image.code_sections().flat_map(move |(start, code)| {
        code.windows(BRANCH_LEN)
            .enumerate()
            .filter_map(move |(offset, window)| {
                let [0xE8 | 0xE9, relative @ ..] = window else {
                    return None;
                };
                let relative = i32::from_le_bytes(relative.try_into().ok()?);
                let branch = start + offset as u32;
                let next_instruction = i64::from(branch.0) + BRANCH_LEN as i64;
                (next_instruction + i64::from(relative) == i64::from(target.0)).then_some(branch)
            })
    })
}

/// Every RIP-relative operand in the code and every pointer in the DLL that refers to `target`.
pub fn references_to(image: &DllImage, target: Rva) -> Vec<Reference> {
    rip_relative_references(image, target)
        .map(Reference::RipRelative)
        .chain(pointers_to(image, target).map(Reference::Pointer))
        .collect()
}

/// A RIP-relative operand is measured from the end of its instruction, which can still have
/// an immediate value of 0, 1, 2 or 4 bytes after the operand (`cmp byte ptr [rip+x],0`).
fn rip_relative_references(image: &DllImage, target: Rva) -> impl Iterator<Item = Rva> + '_ {
    const OPERAND_LEN: usize = 4;
    const IMMEDIATE_LENS: [i64; 4] = [0, 1, 2, 4];

    image.code_sections().flat_map(move |(start, code)| {
        code.windows(OPERAND_LEN)
            .enumerate()
            .filter_map(move |(offset, operand)| {
                let relative = i64::from(i32::from_le_bytes(operand.try_into().ok()?));
                let operand_at = start + offset as u32;
                let operand_end = i64::from(operand_at.0) + OPERAND_LEN as i64;
                IMMEDIATE_LENS
                    .iter()
                    .any(|immediate| operand_end + immediate + relative == i64::from(target.0))
                    .then_some(operand_at)
            })
    })
}

/// Pointers are stored as full addresses that assume the DLL's preferred base address, and
/// are aligned to 8 bytes.
fn pointers_to(image: &DllImage, target: Rva) -> impl Iterator<Item = Rva> + '_ {
    const POINTER_LEN: usize = 8;
    let address = image.image_base() + u64::from(target.0);

    image.all_sections().flat_map(move |(start, bytes)| {
        let (pointers, _rest) = bytes.as_chunks::<POINTER_LEN>();
        pointers
            .iter()
            .enumerate()
            .filter(move |(_, pointer)| u64::from_le_bytes(**pointer) == address)
            .map(move |(index, _)| start + (index * POINTER_LEN) as u32)
    })
}
