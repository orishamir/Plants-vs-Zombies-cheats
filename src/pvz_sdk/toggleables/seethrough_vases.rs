use super::{ToggleCheatError, Toggleable};
use crate::game::Popcapgame;

const RESET_OPACITY_AWAY_OFFSET: [usize; 1] = [0x531cc];
const CHECK_OPACITY_OFFSET: [usize; 1] = [0x527c8];
const DRAW_OPACITY_OFFSET: [usize; 1] = [0x52aa8];

/// ```diff
/// Skip loop that decreases opacity over time:
/// - popcapgame1.exe+531CC - 7E 04                 - jle popcapgame1.exe+531D2
/// + popcapgame1.exe+531CC - EB 04                 - jmp popcapgame1.exe+531D2
///
/// Skip check that opacity > 0:
/// - popcapgame1.exe+527C8 - 0F8E 35030000         - jng popcapgame1.exe+52B03
/// + popcapgame1.exe+527C8 - 9090 90909090         - NOPs
///
/// When drawing opacity, use 50 instead of the actual opacity value:
/// - popcapgame1.exe+52AA8 - 8B 45 4C              - mov eax, [ebp+4C]
/// + popcapgame1.exe+52AA8 - 83 C0 32              - add eax, 32
/// ```
pub struct SeethroughVasesCheat {}

impl Toggleable for SeethroughVasesCheat {
    fn activate(&self, game: &Popcapgame) -> Result<(), ToggleCheatError> {
        game.write::<[u8; _]>(&RESET_OPACITY_AWAY_OFFSET, [0xeb, 0x04])?;
        game.write::<[u8; 6]>(&CHECK_OPACITY_OFFSET, [0x90; _])?;
        game.write::<[u8; _]>(&DRAW_OPACITY_OFFSET, [0x83, 0xc0, 0x50])?;

        Ok(())
    }

    fn deactivate(&self, game: &Popcapgame) -> Result<(), ToggleCheatError> {
        game.write::<[u8; _]>(&RESET_OPACITY_AWAY_OFFSET, [0x7e, 0x04])?;
        game.write::<[u8; _]>(&CHECK_OPACITY_OFFSET, [0x0f, 0x8e, 0x35, 0x03, 0x00, 0x00])?;
        game.write::<[u8; _]>(&DRAW_OPACITY_OFFSET, [0x8b, 0x45, 0x4c])?;

        Ok(())
    }

    fn name(&self) -> &'static str {
        "Seethrough Vases"
    }

    fn is_activated(&self, game: &Popcapgame) -> Result<bool, ToggleCheatError> {
        let current_instructions = game
            .read_bytes_at(game.read_ptr_chain(&RESET_OPACITY_AWAY_OFFSET, true)?, 2)
            .unwrap();

        Ok(current_instructions[0..2] == [0xeb, 0x04])
    }
}
