use super::{Patch, PatchCheat};
use crate::Dll;

/// Seed packets recharge instantly.
///
/// In `Reloaded.Gameplay.SeedPacket$$Update`, `cmp eax,[rbx+0x74]` compares the recharge
/// counter with `SeedPacket.mRefreshTime`, and while there's time left, a jump skips
/// finishing the recharge. Remove the jump.
pub const INSTANT_RECHARGE: PatchCheat = PatchCheat {
    name: "Instant Plant Recharge",
    patches: &[Patch {
        location: Dll::GameAssembly.at(0x6D6C5D),
        original: &[0x7E, 0x1A], // jle +0x1A (still recharging)
        patched: &[0x90; 2],     // nop
    }],
};
