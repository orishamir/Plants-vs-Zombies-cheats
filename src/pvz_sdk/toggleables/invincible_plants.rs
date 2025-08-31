use super::{ToggleCheatError, Toggleable};
use crate::game::Popcapgame;

const INSTRUCTION_OFFSETS: [usize; 1] = [0x1447a0];

#[derive(Default)]
#[allow(dead_code)]
pub struct InvinciblePlantsCheat {}

impl Toggleable for InvinciblePlantsCheat {
    fn activate(&self, process: &Popcapgame) -> Result<(), ToggleCheatError> {
        process
            .write::<[u8; 4]>(&INSTRUCTION_OFFSETS, [0x90; _])
            .unwrap();

        Ok(())
    }

    fn deactivate(&self, process: &Popcapgame) -> Result<(), ToggleCheatError> {
        process
            .write::<[u8; 4]>(
                &INSTRUCTION_OFFSETS,
                [
                    0x83, 0x46, 0x40, 0xFC, // add dword ptr [esi + 0x40], -04
                ],
            )
            .unwrap();

        Ok(())
    }

    fn name(&self) -> &'static str {
        "Invincible Plants"
    }
}
