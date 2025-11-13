use super::{ToggleCheatError, Toggleable};
use crate::Popcapgame;

const INSTRUCTION_OFFSETS: [usize; 1] = [0x681154];

/// ```diff
/// - GameAssembly.dll+681154 - 74 0A                 - je GameAssembly.dll+681160
/// + GameAssembly.dll+681154 - 75 0A                 - jne GameAssembly.dll+681160
/// ```
pub struct AutoPickupSunCheat {}

impl Toggleable for AutoPickupSunCheat {
    #[inline(always)]
    fn name(&self) -> &'static str {
        "Auto Pickup Sun"
    }

    fn activate(&self, process: &Popcapgame) -> Result<(), ToggleCheatError> {
        process.write::<[u8; _]>(&INSTRUCTION_OFFSETS, [0x75, 0x0a])?;

        Ok(())
    }

    fn deactivate(&self, process: &Popcapgame) -> Result<(), ToggleCheatError> {
        process.write::<[u8; _]>(&INSTRUCTION_OFFSETS, [0x74, 0x0a])?;

        Ok(())
    }

    fn is_activated(&self, game: &Popcapgame) -> Result<bool, ToggleCheatError> {
        let current_instructions = game
            .read_bytes_at(game.read_ptr_chain(&INSTRUCTION_OFFSETS, true)?, 2)
            .unwrap();

        Ok(current_instructions[0..2] == [0x75, 0x0a])
    }
}
