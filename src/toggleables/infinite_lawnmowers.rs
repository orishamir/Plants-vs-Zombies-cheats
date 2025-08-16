use super::{ToggleCheatError, Toggleable};
use crate::game::GameProcess;

const INSTRUCTION_OFFSETS: [usize; 1] = [0x5EC4F];

/// Not working
#[derive(Default)]
pub struct InfiniteLawnmowersCheat {}

impl Toggleable for InfiniteLawnmowersCheat {
    fn activate(&self, process: &GameProcess) -> Result<(), ToggleCheatError> {
        Ok(process.write::<[u8; 5]>(
            &INSTRUCTION_OFFSETS.to_vec(),
            [
                0x90, 0x90, 0x90, 0x90, 0x90, // nop
            ],
        )?)
    }

    fn deactivate(&self, process: &GameProcess) -> Result<(), ToggleCheatError> {
        Ok(process.write::<[u8; 5]>(
            &INSTRUCTION_OFFSETS.to_vec(),
            [
                0xE8, 0xCC, 0x02, 0x00, 0x00, // call popcapgame1.exe+5EF20
            ],
        )?)
    }

    fn get_name(&self) -> &'static str {
        "Infinite Lawnmowers"
    }
}
