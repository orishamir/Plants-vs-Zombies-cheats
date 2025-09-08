use super::{ToggleCheatError, Toggleable};
use crate::game::Popcapgame;

const INSTRUCTION_OFFSETS: [usize; 1] = [0x1F634];

pub struct FreePlantsCheat {}

impl Toggleable for FreePlantsCheat {
    fn activate(&self, process: &Popcapgame) -> Result<(), ToggleCheatError> {
        process.write::<[u8; _]>(
            &INSTRUCTION_OFFSETS,
            [
                0x90, 0x90, // NOPs
            ],
        )?;

        Ok(())
    }

    fn deactivate(&self, process: &Popcapgame) -> Result<(), ToggleCheatError> {
        process.write::<[u8; _]>(
            &INSTRUCTION_OFFSETS,
            [
                0x29, 0xde, // sub esi, ebx
            ],
        )?;

        Ok(())
    }

    fn name(&self) -> &'static str {
        "Free Plants"
    }

    fn is_activated(&self, game: &Popcapgame) -> Result<bool, ToggleCheatError> {
        let current_instructions = game
            .read_bytes_at(game.read_ptr_chain(&INSTRUCTION_OFFSETS, true)?, 2)
            .unwrap();

        Ok(current_instructions[0..2] == [0x90, 0x90])
    }
}
