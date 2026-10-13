use super::{Patch, PatchCheat};
use crate::Dll;

/// Every coin, sun included, flies to the counter as soon as it appears.
///
/// `Reloaded.Gameplay.Coin$$Update` either moves a coin normally (`Coin$$UpdateFall`) or flies
/// it to the counter (`Coin$$UpdateCollected`), depending on whether it's being collected:
/// `cmp [rbx+0xA8],dil` compares `Coin.mIsBeingCollected` with 0. Inverting the jump after
/// that comparison collects every coin.
pub const AUTO_PICKUP_SUN: PatchCheat = PatchCheat {
    name: "Auto Pickup Sun",
    patches: &[Patch {
        location: Dll::GameAssembly.at(0x69A6B5),
        original: &[0x74, 0x0A], // je +0x0A (to UpdateFall)
        patched: &[0x75, 0x0A],  // jne +0x0A
    }],
};
