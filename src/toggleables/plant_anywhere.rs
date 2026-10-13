use super::{Patch, PatchCheat};
use crate::Dll;

/// Plants can go on any tile, even where the game normally refuses.
///
/// `Reloaded.Gameplay.Board$$MouseDownWithPlant` calls `Board$$CanPlantAt` to check the
/// clicked tile. It returns a `PlantingReason`, where 0 means the plant is allowed. Right after
/// the call, pretend it returned 0.
///
/// Other checks still use `CanPlantAt` as usual. For example, the plant preview under the
/// cursor may not show on tiles the game refuses, and in "Column Like You See 'Em" the rest of
/// the column is checked by a second call at `MouseDownWithPlant+0xDC9`.
pub const PLANT_ANYWHERE: PatchCheat = PatchCheat {
    name: "Plant Anywhere",
    patches: &[Patch {
        location: Dll::GameAssembly.at(0x6657E7),
        original: &[
            0x8B, 0xD8, // mov ebx,eax
            0x85, 0xC0, // test eax,eax
        ],
        patched: &[
            0x31, 0xDB, // xor ebx,ebx
            0x31, 0xC0, // xor eax,eax
        ],
    }],
};
