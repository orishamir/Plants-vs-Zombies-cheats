pub(crate) mod cheated_entity;
pub mod entities;
pub(crate) mod game;
mod location;
pub(crate) mod offsets;
pub(crate) mod readers;
pub mod toggleables;
pub(crate) mod writers;

pub use game::{MemoryError, Popcapgame};
pub use location::{Dll, Location};

pub mod traits {
    pub use super::{
        readers::{MemoryReaderError, ReadEntityError, ReadableEntity},
        toggleables::{ToggleCheatError, Toggleable},
        writers::WriteableEntity,
    };
}
