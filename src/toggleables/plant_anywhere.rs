use super::{ToggleCheatError, Toggleable};
use crate::game::GameProcess;

const INSTRUCTION_OFFSETS: [usize; 1] = [0x1334D];

#[derive(Default)]
pub struct PlantAnywhereCheat {}

impl Toggleable for PlantAnywhereCheat {
    fn activate(&self, process: &GameProcess) -> Result<(), ToggleCheatError> {
        Ok(process.write::<[u8; 2]>(
            &INSTRUCTION_OFFSETS,
            [
                0x31, 0xC0, // xor eax, eax
            ],
        )?)
    }

    fn deactivate(&self, process: &GameProcess) -> Result<(), ToggleCheatError> {
        Ok(process.write::<[u8; 2]>(
            &INSTRUCTION_OFFSETS,
            [
                0x85, 0xC0, // test eax, eax
            ],
        )?)
    }

    fn name(&self) -> &'static str {
        "Plant Anywhere"
    }
}
