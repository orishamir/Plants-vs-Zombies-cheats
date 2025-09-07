use super::{ToggleCheatError, Toggleable};
use crate::game::Popcapgame;

const INSTRUCTION_OFFSETS: [usize; 1] = [0x1334D];

pub struct PlantAnywhereCheat {}

impl Toggleable for PlantAnywhereCheat {
    fn activate(&self, process: &Popcapgame) -> Result<(), ToggleCheatError> {
        process.write::<[u8; _]>(
            &INSTRUCTION_OFFSETS,
            [
                0x31, 0xC0, // xor eax, eax
            ],
        )?;

        Ok(())
    }

    fn deactivate(&self, process: &Popcapgame) -> Result<(), ToggleCheatError> {
        process.write::<[u8; _]>(
            &INSTRUCTION_OFFSETS,
            [
                0x85, 0xC0, // test eax, eax
            ],
        )?;

        Ok(())
    }

    fn name(&self) -> &'static str {
        "Plant Anywhere"
    }
}
