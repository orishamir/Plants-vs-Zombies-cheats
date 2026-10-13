use super::{Patch, PatchCheat};
use crate::Dll;

/// Any hit takes all of a zombie's health: its body, its armor (like a cone or bucket) and
/// its shield (like a screen door).
pub const HEADSHOT_MODE: PatchCheat = PatchCheat {
    name: "Headshot Mode",
    patches: &[
        // `Reloaded.Gameplay.Zombie$$TakeBodyDamage` subtracts the damage (`edi`) from what
        // becomes `Zombie.mBodyHealth` (`+0x120`). Subtract all of it instead.
        Patch {
            location: Dll::GameAssembly.at(0x6F85D5),
            original: &[0x2B, 0xEF], // sub ebp,edi
            patched: &[0x29, 0xED],  // sub ebp,ebp
        },
        // `Reloaded.Gameplay.Zombie$$TakeHelmDamage` subtracts the damage, capped at what's
        // left, from `Zombie.mHelmHealth` (`+0x128`). Set it to 0 instead.
        Patch {
            location: Dll::GameAssembly.at(0x6F939D),
            original: &[
                0x0F, 0x4C, 0xC8, // cmovl ecx,eax
                0x29, 0x8B, 0x28, 0x01, 0x00, 0x00, // sub [rbx+0x128],ecx
            ],
            patched: &[
                0x31, 0xC9, // xor ecx,ecx
                0x90, // nop
                0x89, 0x8B, 0x28, 0x01, 0x00, 0x00, // mov [rbx+0x128],ecx
            ],
        },
        // `Reloaded.Gameplay.Zombie$$TakeShieldDamage` does the same for
        // `Zombie.mShieldHealth` (`+0x134`).
        Patch {
            location: Dll::GameAssembly.at(0x6F99D2),
            original: &[
                0x2B, 0xF2, // sub esi,edx
                0x29, 0x93, 0x34, 0x01, 0x00, 0x00, // sub [rbx+0x134],edx
            ],
            patched: &[
                0x31, 0xD2, // xor edx,edx
                0x89, 0x93, 0x34, 0x01, 0x00, 0x00, // mov [rbx+0x134],edx
            ],
        },
    ],
};
