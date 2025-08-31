use super::{ToggleCheatError, Toggleable};
use crate::game::Popcapgame;

const INSTRUCTION_OFFSETS: [usize; 1] = [0x1F634];

#[derive(Default)]
#[allow(dead_code)]
pub struct FreePlantsCheat {}

impl Toggleable for FreePlantsCheat {
    fn activate(&self, process: &Popcapgame) -> Result<(), ToggleCheatError> {
        process
            .write::<[u8; 2]>(
                &INSTRUCTION_OFFSETS,
                [
                    0x90, 0x90, // NOPs
                ],
            )
            .unwrap();

        Ok(())
    }

    fn deactivate(&self, process: &Popcapgame) -> Result<(), ToggleCheatError> {
        process
            .write::<[u8; _]>(
                &INSTRUCTION_OFFSETS,
                [
                    0x29, 0xde, // sub esi, ebx
                ],
            )
            .unwrap();

        Ok(())
    }

    fn name(&self) -> &'static str {
        "Free Plants"
    }
}
