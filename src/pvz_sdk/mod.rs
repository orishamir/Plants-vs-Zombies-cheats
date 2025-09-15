pub(crate) mod cheated_entity;
pub mod entities;
pub(crate) mod game;
pub(crate) mod offsets;
pub(crate) mod readers;
pub mod toggleables;
pub(crate) mod writers;

pub use game::Popcapgame;

pub mod traits {
    pub use super::{
        readers::{MemoryReaderError, ReadEntityError, ReadableEntity},
        toggleables::{ToggleCheatError, Toggleable},
        writers::WriteableEntity,
    };
}
