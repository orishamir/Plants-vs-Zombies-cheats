use super::{ToggleCheatError, Toggleable};
use crate::game::GameProcess;

const INSTRUCTION_OFFSETS: [usize; 1] = [0x958BC];

#[derive(Default)]
pub struct InstantRechargeCheat {}

impl Toggleable for InstantRechargeCheat {
    fn activate(&self, process: &GameProcess) -> Result<(), ToggleCheatError> {
        Ok(process.write::<[u8; 9]>(
            &INSTRUCTION_OFFSETS,
            [
                0x81, 0x47, 0x24, 0x0, 0x2, 0x0, 0x0,  // add [edi+24], 000200
                0x90, // NOP
                0x90, // NOP
            ],
        )?)
    }

    fn deactivate(&self, process: &GameProcess) -> Result<(), ToggleCheatError> {
        Ok(process.write::<[u8; 9]>(
            &INSTRUCTION_OFFSETS,
            [
                0xff, 0x47, 0x24, // inc [edi + 24]
                0x8b, 0x47, 0x24, // mov eax, [edi+24]
                0x3b, 0x47, 0x28, // cmp eax, [edi+28]
            ],
        )?)
    }

    fn get_name(&self) -> &'static str {
        "Instant Plant Recharge"
    }
}
