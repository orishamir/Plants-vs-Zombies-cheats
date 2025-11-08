use super::{ToggleCheatError, Toggleable};
use crate::Popcapgame;

const DEC_OPACITY_OFFSET: [usize; 1] = [0x6B706F];
const CHECK_OPACITY_OFFSET: [usize; 1] = [0x7BF7A5];
const DRAW_OPACITY_OFFSET: [usize; 1] = [0x7BFA43];

/// ```diff
/// Don't decrement opacity over time:
/// - GameAssembly.dll+6B706F - FF 8B B0000000        - dec [rbx+000000B0]
/// + GameAssembly.dll+6B706F - 90 90 90909090        - nop
///
/// Skip check that opacity > 0:
/// - GameAssembly.dll+7BF7A5 - 7F 23                 - jg GameAssembly.dll+7BF7CA
/// + GameAssembly.dll+7BF7A5 - EB 23                 - jmp GameAssembly.dll+7BF7CA
///
/// When drawing opacity, use 50 instead of the actual opacity value:
/// - GameAssembly.dll+7BFA43 - 8B B8 B0000000        - mov edi,[rax+000000B0]
/// + GameAssembly.dll+7BFA43 - BF 32000000           - mov edi,00000032
/// + GameAssembly.dll+7BFA48 - 90                    - nop
/// ```
pub struct SeethroughVasesCheat {}

impl Toggleable for SeethroughVasesCheat {
    #[inline(always)]
    fn name(&self) -> &'static str {
        "Seethrough Vases"
    }

    fn activate(&self, game: &Popcapgame) -> Result<(), ToggleCheatError> {
        game.write::<[u8; 6]>(&DEC_OPACITY_OFFSET, [0x90; _])?;
        game.write::<[u8; _]>(&CHECK_OPACITY_OFFSET, [0xeb, 0x23])?;
        game.write::<[u8; _]>(&DRAW_OPACITY_OFFSET, [0xbf, 0x32, 0x0, 0x0, 0x0, 0x90])?;

        Ok(())
    }

    fn deactivate(&self, game: &Popcapgame) -> Result<(), ToggleCheatError> {
        game.write::<[u8; _]>(&DEC_OPACITY_OFFSET, [0xFF, 0x8B, 0xB0, 0x0, 0x0, 0x0])?;
        game.write::<[u8; _]>(&CHECK_OPACITY_OFFSET, [0x7f, 0x23])?;
        game.write::<[u8; _]>(&DRAW_OPACITY_OFFSET, [0x8b, 0xb8, 0xb0, 0x0, 0x0, 0x0])?;

        Ok(())
    }

    fn is_activated(&self, game: &Popcapgame) -> Result<bool, ToggleCheatError> {
        let current_instructions = game
            .read_bytes_at(game.read_ptr_chain(&DEC_OPACITY_OFFSET, true)?, 2)
            .unwrap();

        Ok(current_instructions[0..2] == [0x90, 0x90])
    }
}
