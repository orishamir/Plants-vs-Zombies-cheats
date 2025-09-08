use super::{ToggleCheatError, Toggleable};
use crate::game::Popcapgame;

const INSTRUCTION_OFFSETS: [usize; 1] = [0x1334D];

pub struct PlantAnywhereCheat {}

impl Toggleable for PlantAnywhereCheat {
    #[inline(always)]
    fn name(&self) -> &'static str {
        "Plant Anywhere"
    }

    fn activate(&self, game: &Popcapgame) -> Result<(), ToggleCheatError> {
        game.write::<[u8; _]>(
            &INSTRUCTION_OFFSETS,
            [
                0x31, 0xC0, // xor eax, eax
            ],
        )?;

        Ok(())
    }

    fn deactivate(&self, game: &Popcapgame) -> Result<(), ToggleCheatError> {
        game.write::<[u8; _]>(
            &INSTRUCTION_OFFSETS,
            [
                0x85, 0xC0, // test eax, eax
            ],
        )?;

        Ok(())
    }

    fn is_activated(&self, game: &Popcapgame) -> Result<bool, ToggleCheatError> {
        let current_instructions = game
            .read_bytes_at(game.read_ptr_chain(&INSTRUCTION_OFFSETS, true)?, 2)
            .unwrap();

        Ok(current_instructions[0..2] == [0x31, 0xc0])
    }
}
