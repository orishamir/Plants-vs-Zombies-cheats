use super::{ToggleCheatError, Toggleable};
use crate::Popcapgame;

const NORMAL_OFFSET: [usize; 1] = [0x6EA165];
const ARMOR_OFFSET: [usize; 1] = [0x6EAF2D];
const SHIELD_OFFSET: [usize; 1] = [0x6eb55b];

/// Normal zombie damage
/// ```diff
/// - GameAssembly.dll+6EA165 - 29 FD                 - sub ebp,edi
/// + GameAssembly.dll+6EA165 - 29 ED                 - sub ebp,ebp
/// ```
///
/// Also insta-kill armor
/// ```diff
/// - GameAssembly.dll+6EAF2D - 0F4C C8               - cmovl ecx,eax
/// - GameAssembly.dll+6EAF30 - 29 8B 28010000        - sub [rbx+00000128],ecx
/// + GameAssembly.dll+6EAF2D - 31 C9                 - xor ecx,ecx
/// + GameAssembly.dll+6EAF2F - 90                    - nop
/// + GameAssembly.dll+6EAF30 - 89 8B 28010000        - mov [rbx+00000128],ecx
/// ```
///
/// And insta kill shield
/// ```diff
/// - GameAssembly.dll+6EB55B - 2B F2                 - sub esi,edx
/// - GameAssembly.dll+6EB55D - 29 93 34010000        - sub [rbx+00000134],edx
/// + GameAssembly.dll+6EB55B - 31 D2                 - xor edx,edx
/// + GameAssembly.dll+6EB55D - 89 93 34010000        - mov [rbx+00000134],edx
/// ```
pub struct HeadshotMode {}

impl Toggleable for HeadshotMode {
    #[inline(always)]
    fn name(&self) -> &'static str {
        "Headshot Mode"
    }

    fn activate(&self, process: &Popcapgame) -> Result<(), ToggleCheatError> {
        process.write::<[u8; _]>(&NORMAL_OFFSET, [0x29, 0xed])?;

        process.write::<[u8; _]>(
            &ARMOR_OFFSET,
            [
                0x31, 0xc9, // first
                0x90, // nop
                0x89, 0x8b, 0x28, 0x01, 0x0, 0x0,
            ],
        )?;

        process.write::<[u8; _]>(
            &SHIELD_OFFSET,
            [
                0x31, 0xd2, // first
                0x89, 0x93, 0x34, 0x01, 0x0, 0x0,
            ],
        )?;

        Ok(())
    }

    fn deactivate(&self, process: &Popcapgame) -> Result<(), ToggleCheatError> {
        process.write::<[u8; _]>(&NORMAL_OFFSET, [0x29, 0xfd])?;

        process.write::<[u8; _]>(
            &ARMOR_OFFSET,
            [
                0x0f, 0x4c, 0xc8, // first
                0x29, 0x8B, 0x28, 0x01, 0x00, 0x00,
            ],
        )?;

        process.write::<[u8; _]>(
            &SHIELD_OFFSET,
            [
                0x2b, 0xf2, // first
                0x29, 0x93, 0x34, 0x01, 0x00, 0x00,
            ],
        )?;

        Ok(())
    }

    fn is_activated(&self, game: &Popcapgame) -> Result<bool, ToggleCheatError> {
        let current_instructions = game
            .read_bytes_at(game.read_ptr_chain(&NORMAL_OFFSET, true)?, 2)
            .unwrap();

        Ok(current_instructions[0..2] == [0x29, 0xed])
    }
}
