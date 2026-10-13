use super::{Patch, PatchCheat};
use crate::Dll;

/// Chompers don't digest: they can bite again right after eating a zombie.
///
/// When a chomper catches a zombie, the game sets its state (`Plant.mState`, `+0x88`) to
/// [`PlantState::ChomperBitingGotOne`](crate::entities::PlantState::ChomperBitingGotOne), which
/// leads to the long `ChomperDigesting`. Skipping that write leaves the chomper able to bite
/// again.
///
/// The code is `Plant$$UpdateChomper`, which the compiler copied into
/// `Reloaded.Gameplay.Plant$$UpdateAbilities`. The standalone `Plant$$UpdateChomper` has the
/// same instruction at `+0x2F0`, but nothing calls it.
pub const HUNGRY_CHOMPERS: PatchCheat = PatchCheat {
    name: "Hungry Chompers",
    patches: &[Patch {
        location: Dll::GameAssembly.at(0x6B77B1),
        // mov dword ptr [rbx+0x88],11 (ChomperBitingGotOne)
        original: &[0xC7, 0x83, 0x88, 0x00, 0x00, 0x00, 0x0B, 0x00, 0x00, 0x00],
        patched: &[0x90; 10], // nop
    }],
};
