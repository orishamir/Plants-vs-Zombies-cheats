use super::{ToggleCheatError, Toggleable};
use crate::Popcapgame;

const INSTRUCTION_OFFSETS: [usize; 1] = [0x1447a0];

pub struct InvinciblePlantsCheat {}

impl Toggleable for InvinciblePlantsCheat {
    #[inline(always)]
    fn name(&self) -> &'static str {
        "Invincible Plants"
    }

    fn activate(&self, game: &Popcapgame) -> Result<(), ToggleCheatError> {
        game.write::<[u8; 4]>(&INSTRUCTION_OFFSETS, [0x90; _])?;

        Ok(())
    }

    fn deactivate(&self, process: &Popcapgame) -> Result<(), ToggleCheatError> {
        process.write::<[u8; _]>(
            &INSTRUCTION_OFFSETS,
            [
                0x83, 0x46, 0x40, 0xFC, // add dword ptr [esi + 0x40], -04
            ],
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
