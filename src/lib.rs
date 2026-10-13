pub mod entities;
pub(crate) mod game;
pub mod il2cpp;
mod location;
pub mod toggleables;

pub use game::{MemoryError, Popcapgame};
pub use location::{Dll, Location};

pub mod traits {
    pub use super::toggleables::{ToggleCheatError, Toggleable};
}
