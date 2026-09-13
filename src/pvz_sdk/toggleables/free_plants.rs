use super::{ToggleCheatError, Toggleable};
use crate::Popcapgame;

const INSTRUCTION_OFFSETS: [usize; 1] = [0x665DE0];

/// ```diff
/// - GameAssembly.dll+665DE0 - 44 29 60 10           - sub [rax+10],r12d
/// + GameAssembly.dll+665DE0 - 90 90 90 90           - nop
/// ```
pub struct FreePlantsCheat {}

impl Toggleable for FreePlantsCheat {
    #[inline(always)]
    fn name(&self) -> &'static str {
        "Free Plants"
    }

    fn activate(&self, process: &Popcapgame) -> Result<(), ToggleCheatError> {
        process.write::<[u8; 4]>(&INSTRUCTION_OFFSETS, [0x90; _])?;

        Ok(())
    }

    fn deactivate(&self, process: &Popcapgame) -> Result<(), ToggleCheatError> {
        process.write::<[u8; _]>(&INSTRUCTION_OFFSETS, [0x44, 0x29, 0x60, 0x10])?;

        Ok(())
    }

    fn is_activated(&self, game: &Popcapgame) -> Result<bool, ToggleCheatError> {
        let current_instructions = game
            .read_bytes_at(game.read_ptr_chain(&INSTRUCTION_OFFSETS, true)?, 2)
            .unwrap();

        Ok(current_instructions[0..2] == [0x90, 0x90])
    }
}
