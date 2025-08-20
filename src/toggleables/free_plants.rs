use super::{ToggleCheatError, Toggleable};
use crate::game::GameProcess;

const INSTRUCTION_OFFSETS: [usize; 1] = [0x1F634];

#[derive(Default)]
pub struct FreePlantsCheat {}

impl Toggleable for FreePlantsCheat {
    fn activate(&self, process: &GameProcess) -> Result<(), ToggleCheatError> {
        Ok(process.write::<[u8; 2]>(
            &INSTRUCTION_OFFSETS,
            [
                0x90, 0x90, // NOPs
            ],
        )?)
    }

    fn deactivate(&self, process: &GameProcess) -> Result<(), ToggleCheatError> {
        Ok(process.write::<[u8; _]>(
            &INSTRUCTION_OFFSETS,
            [
                0x29, 0xde, // sub esi, ebx
            ],
        )?)
    }

    fn name(&self) -> &'static str {
        "Free Plants"
    }
}
