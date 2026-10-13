use super::{Patch, PatchCheat};
use crate::Dll;

/// Zombies' bites don't hurt plants.
///
/// In `Reloaded.Gameplay.Zombie$$EatPlant`, each bite takes 4 health from the plant
/// (`Plant.mPlantHealth`, `+0x8C`). Skip it.
///
/// The same instruction appears again at `Zombie$$EatPlant+0x26D`. That's an extra bite on
/// wall-nuts, tall-nuts and pumpkins in I, Zombie levels, so it's left alone.
pub const INVINCIBLE_PLANTS: PatchCheat = PatchCheat {
    name: "Invincible Plants",
    patches: &[Patch {
        location: Dll::GameAssembly.at(0x6F10D6),
        original: &[0x83, 0xAB, 0x8C, 0x00, 0x00, 0x00, 0x04], // sub dword ptr [rbx+0x8C],4
        patched: &[0x90; 7],                                   // nop
    }],
};
