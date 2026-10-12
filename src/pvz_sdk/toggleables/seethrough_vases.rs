use super::{Patch, PatchCheat};
use crate::Dll;

/// Vases always show what's inside them.
///
/// A vase shows its contents while its opacity (`GridItem.mTransparentCounter`, `+0xB0`) is
/// above 0. Normally only a nearby Plantern raises it.
pub const SEETHROUGH_VASES: PatchCheat = PatchCheat {
    name: "Seethrough Vases",
    patches: &[
        // Don't lower the opacity over time. This is `GridItem$$UpdateScaryPot`, which the
        // compiler copied into `Reloaded.Gameplay.GridItem$$Update` (nothing calls the original).
        Patch {
            location: Dll::GameAssembly.at(0x6C311F),
            original: &[0xFF, 0x8B, 0xB0, 0x00, 0x00, 0x00], // dec dword ptr [rbx+0xB0]
            patched: &[0x90; 6],                             // nop
        },
        // The rest is in `Source.Controllers.ScaryPotController$$Update`, which draws the
        // vase. Both checks follow a `cmp [rax+0xB0],r15d`, where `r15d` is 0.
        //
        // Show a sun vase's sun even at opacity 0.
        Patch {
            location: Dll::GameAssembly.at(0x7CBE44),
            original: &[0x7F, 0x05], // jg +0x05
            patched: &[0xEB, 0x05],  // jmp +0x05
        },
        // Show the plant or zombie inside even at opacity 0.
        Patch {
            location: Dll::GameAssembly.at(0x7CBEA5),
            original: &[0x7F, 0x7D], // jg +0x7D
            patched: &[0xEB, 0x7D],  // jmp +0x7D
        },
        // Draw the contents at opacity 50 instead of the actual value.
        Patch {
            location: Dll::GameAssembly.at(0x7CC206),
            original: &[0x8B, 0xB8, 0xB0, 0x00, 0x00, 0x00], // mov edi,[rax+0xB0]
            patched: &[
                0xBF, 0x32, 0x00, 0x00, 0x00, // mov edi,50
                0x90, // nop
            ],
        },
    ],
};
