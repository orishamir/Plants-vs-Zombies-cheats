use super::{ToggleCheatError, Toggleable};
use crate::game::GameProcess;

const INSTRUCTION_OFFSETS: [usize; 1] = [0x1447a0];

#[derive(Default)]
pub struct InvinciblePlantsCheat {}

impl Toggleable for InvinciblePlantsCheat {
    fn activate(&self, process: &GameProcess) -> Result<(), ToggleCheatError> {
        process.write::<[u8; 4]>(&INSTRUCTION_OFFSETS, [0x90; _]);

        Ok(())
    }

    fn deactivate(&self, process: &GameProcess) -> Result<(), ToggleCheatError> {
        process.write::<[u8; 4]>(
            &INSTRUCTION_OFFSETS,
            [
                0x83, 0x46, 0x40, 0xFC, // add dword ptr [esi + 0x40], -04
            ],
        );

        Ok(())
    }

    fn name(&self) -> &'static str {
        "Invincible Plants"
    }
}
