//! Cheats that can be turned on and off.
//!
//! Each cheat is a [`PatchCheat`]: a name and the [`Patch`]es it makes to the game's code.
//! To add one, write it in its own file and add it to [`ALL`].

mod auto_pickup_sun;
mod free_plants;
mod headshot_mode;
mod hungry_chomper;
mod instant_recharge;
mod invincible_plants;
mod no_pause;
mod patch;
mod plant_anywhere;
mod seethrough_vases;
mod toggleable;

pub use auto_pickup_sun::AUTO_PICKUP_SUN;
pub use free_plants::FREE_PLANTS;
pub use headshot_mode::HEADSHOT_MODE;
pub use hungry_chomper::HUNGRY_CHOMPERS;
pub use instant_recharge::INSTANT_RECHARGE;
pub use invincible_plants::INVINCIBLE_PLANTS;
pub use no_pause::NO_PAUSE;
pub use patch::{Patch, PatchCheat};
pub use plant_anywhere::PLANT_ANYWHERE;
pub use seethrough_vases::SEETHROUGH_VASES;
pub use toggleable::{ToggleCheatError, Toggleable};

/// Every cheat, in the order the GUI shows them.
pub const ALL: &[PatchCheat] = &[
    AUTO_PICKUP_SUN,
    INSTANT_RECHARGE,
    FREE_PLANTS,
    INVINCIBLE_PLANTS,
    SEETHROUGH_VASES,
    HEADSHOT_MODE,
    HUNGRY_CHOMPERS,
    PLANT_ANYWHERE,
    NO_PAUSE,
];

#[cfg(test)]
mod tests {
    use super::ALL;

    #[test]
    fn every_patch_replaces_as_many_bytes_as_it_writes() {
        for cheat in ALL {
            for patch in cheat.patches {
                let at = format!("{} at {}", cheat.name, patch.location);
                assert_eq!(patch.original.len(), patch.patched.len(), "{at}");
                assert_ne!(patch.original, patch.patched, "{at}");
            }
        }
    }
}
