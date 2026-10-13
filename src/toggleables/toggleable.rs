use thiserror::Error;

use crate::{Game, Location, MemoryError};

/// A cheat that can be turned on and off.
pub trait Toggleable {
    fn activate(&self, game: &Game) -> Result<(), ToggleCheatError>;
    fn deactivate(&self, game: &Game) -> Result<(), ToggleCheatError>;
    fn is_active(&self, game: &Game) -> Result<bool, ToggleCheatError>;
    fn name(&self) -> &'static str;

    /// Turns the cheat on or off.
    fn set_active(&self, game: &Game, active: bool) -> Result<(), ToggleCheatError> {
        if active {
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
