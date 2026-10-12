use thiserror::Error;

use crate::{Location, MemoryError, Popcapgame};

/// A cheat that can be turned on and off.
pub trait Toggleable {
    fn activate(&self, game: &Popcapgame) -> Result<(), ToggleCheatError>;
    fn deactivate(&self, game: &Popcapgame) -> Result<(), ToggleCheatError>;
    fn is_activated(&self, game: &Popcapgame) -> Result<bool, ToggleCheatError>;
    fn name(&self) -> &'static str;
    fn toggle(&self, game: &Popcapgame, toggled: bool) -> Result<(), ToggleCheatError> {
        if toggled {
            self.activate(game)
        } else {
            self.deactivate(game)
        }
    }
}

#[derive(Debug, Error)]
pub enum ToggleCheatError {
    #[error(transparent)]
    Memory(#[from] MemoryError),

    /// The game's code isn't what a patch expects, usually because a game update moved it.
    #[error(
        "expected {expected:02X?} at {location} but found {found:02X?}. \
         The game was probably updated; tools/re-tools/README.md explains how to find the patch again"
    )]
    UnexpectedBytes {
        location: Location,
        expected: &'static [u8],
        found: Vec<u8>,
    },
}
