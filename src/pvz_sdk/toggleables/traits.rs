use proc_mem::ProcMemError;
use thiserror::Error;

use crate::Popcapgame;

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
    #[error("A memory error occured")]
    MemoryError(ProcMemError),
}

impl From<ProcMemError> for ToggleCheatError {
    fn from(value: ProcMemError) -> Self {
        Self::MemoryError(value)
    }
}
