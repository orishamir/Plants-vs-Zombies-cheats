use super::{ToggleCheatError, Toggleable};
use crate::Popcapgame;

const INSTRUCTION_OFFSETS: [usize; 1] = [0x6E2A96];
/// ```diff
/// - GameAssembly.dll+6E2A96 - 83 AB 8C000000 04        - sub dword ptr [rbx+0000008C],04
/// + GameAssembly.dll+6E2A96 - 90 90 90 90 90 90 90     - nop
/// ```
pub struct InvinciblePlantsCheat {}

impl Toggleable for InvinciblePlantsCheat {
    #[inline(always)]
    fn name(&self) -> &'static str {
        "Invincible Plants"
    }

    fn activate(&self, game: &Popcapgame) -> Result<(), ToggleCheatError> {
        game.write::<[u8; 7]>(&INSTRUCTION_OFFSETS, [0x90; _])?;

        Ok(())
    }

    fn deactivate(&self, process: &Popcapgame) -> Result<(), ToggleCheatError> {
        process.write::<[u8; _]>(
            &INSTRUCTION_OFFSETS,
            [0x83, 0xab, 0x8c, 0x0, 0x0, 0x0, 0x4], // sub dword ptr [rbx+0000008C],04
        )?;

        Ok(())
    }

    fn is_activated(&self, game: &Popcapgame) -> Result<bool, ToggleCheatError> {
        let current_instructions = game
            .read_bytes_at(game.read_ptr_chain(&INSTRUCTION_OFFSETS, true)?, 4)
            .unwrap();

        Ok(current_instructions[0..4] == [0x90, 0x90, 0x90, 0x90])
    }
}
