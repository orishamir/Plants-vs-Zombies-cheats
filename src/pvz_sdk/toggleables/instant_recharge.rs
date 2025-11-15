use super::{ToggleCheatError, Toggleable};
use crate::Popcapgame;

const INSTRUCTION_OFFSETS: [usize; 1] = [0x6CB1FD];

/// ```diff
///   GameAssembly.dll+6CB1FA - 3B 43 74              - cmp slot->reloadCounter+1, slot->reloadDestination
/// - GameAssembly.dll+6CB1FD - 7E 1A                 - jle GameAssembly.dll+6CB219   (If hasn't reached reload destination, reload didn't finish)
/// + GameAssembly.dll+6CB1FD - 90 90                 - nop
/// ```
pub struct InstantRechargeCheat {}

impl Toggleable for InstantRechargeCheat {
    #[inline(always)]
    fn name(&self) -> &'static str {
        "Instant Plant Recharge"
    }

    fn activate(&self, game: &Popcapgame) -> Result<(), ToggleCheatError> {
        game.write::<[u8; _]>(&INSTRUCTION_OFFSETS, [0x90, 0x90])?;

        Ok(())
    }

    fn deactivate(&self, process: &Popcapgame) -> Result<(), ToggleCheatError> {
        process.write::<[u8; _]>(&INSTRUCTION_OFFSETS, [0x7e, 0x1a])?;

        Ok(())
    }

    fn is_activated(&self, game: &Popcapgame) -> Result<bool, ToggleCheatError> {
        let current_instructions = game
            .read_bytes_at(game.read_ptr_chain(&INSTRUCTION_OFFSETS, true)?, 2)
            .unwrap();

        Ok(current_instructions[0..2] == [0x90, 0x90])
    }
}
