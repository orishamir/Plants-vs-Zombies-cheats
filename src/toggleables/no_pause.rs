use super::{ToggleCheatError, Toggleable};
use crate::game::Popcapgame;

const INSTRUCTION_OFFSETS: [usize; 1] = [0x1924B];

/// The cheat modifies the instructions like this:
/// ```diff
/// popcapgame1.exe+19244 - 80 BD 7C010000 00     - cmp byte ptr [ebp+0000017C],00
/// - popcapgame1.exe+1924B - 74 37                 - je popcapgame1.exe+19284
/// + popcapgame1.exe+1924B - EB 37                 - jmp popcapgame1.exe+19284
/// ```
/// Note: [esi+0000017C] points to the value of "is_paused"
/// So, translated to C:
/// ```c
/// if (!is_paused) run_game()
/// ```
/// I change it to run regardless.
///
/// TODO: Hide "pause" menu
#[derive(Default)]
pub struct NoPauseCheat {}

impl Toggleable for NoPauseCheat {
    fn activate(&self, process: &Popcapgame) -> Result<(), ToggleCheatError> {
        process
            .write::<[u8; 2]>(
                &INSTRUCTION_OFFSETS,
                [
                    0xEB, 0x37, // jmp popcapgame1.exe+19284
                ],
            )
            .unwrap();
        Ok(())
    }

    fn deactivate(&self, process: &Popcapgame) -> Result<(), ToggleCheatError> {
        process
            .write::<[u8; 2]>(
                &INSTRUCTION_OFFSETS,
                [
                    0x74, 0x37, // je popcapgame1.exe+19284
                ],
            )
            .unwrap();

        Ok(())
    }

    fn name(&self) -> &'static str {
        "No Pause"
    }
}
