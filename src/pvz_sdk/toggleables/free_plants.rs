use super::{Patch, PatchCheat};
use crate::Dll;

/// Planting doesn't cost sun.
///
/// `Reloaded.Gameplay.Board$$MouseDownWithPlant` subtracts the seed's cost from the sun count
/// when you plant. Skip the subtraction.
pub const FREE_PLANTS: PatchCheat = PatchCheat {
    name: "Free Plants",
    patches: &[Patch {
        location: Dll::GameAssembly.at(0x665DE0),
        original: &[0x44, 0x29, 0x60, 0x10], // sub [rax+0x10],r12d
        patched: &[0x90; 4],                 // nop
    }],
};
