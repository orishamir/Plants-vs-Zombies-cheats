use super::{ToggleCheatError, Toggleable};
use crate::game::Popcapgame;

const INSTRUCTION_OFFSETS: [usize; 1] = [0x958BC];

pub struct InstantRechargeCheat {}

impl Toggleable for InstantRechargeCheat {
    fn activate(&self, game: &Popcapgame) -> Result<(), ToggleCheatError> {
        game.write::<[u8; _]>(
            &INSTRUCTION_OFFSETS,
            [
                0x81, 0x47, 0x24, 0x0, 0x2, 0x0, 0x0,  // add [edi+24], 000200
                0x90, // NOP
                0x90, // NOP
            ],
        )?;

        Ok(())
    }

    fn deactivate(&self, process: &Popcapgame) -> Result<(), ToggleCheatError> {
        process.write::<[u8; _]>(
            &INSTRUCTION_OFFSETS,
            [
                0xff, 0x47, 0x24, // inc [edi + 24]
                0x8b, 0x47, 0x24, // mov eax, [edi+24]
                0x3b, 0x47, 0x28, // cmp eax, [edi+28]
            ],
        )?;

        Ok(())
    }

    fn name(&self) -> &'static str {
        "Instant Plant Recharge"
    }

    fn is_activated(&self, game: &Popcapgame) -> Result<bool, ToggleCheatError> {
        let current_instructions = game
            .read_bytes_at(game.read_ptr_chain(&INSTRUCTION_OFFSETS, true)?, 4)
            .unwrap();

        Ok(current_instructions[0..4] == [0x81, 0x47, 0x24, 0x0])
    }
}
