use proc_mem::ProcMemError;
use thiserror::Error;

use crate::game::Popcapgame;

#[allow(dead_code)]
pub trait Toggleable {
    fn activate(&self, process: &Popcapgame) -> Result<(), ToggleCheatError>;
    fn deactivate(&self, process: &Popcapgame) -> Result<(), ToggleCheatError>;
    fn name(&self) -> &'static str;
    fn toggle(&mut self, process: &Popcapgame, toggled: bool) -> Result<(), ToggleCheatError> {
        if toggled {
            self.activate(process)
        } else {
            self.deactivate(process)
        }
    }
}

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum ToggleCheatError {
    #[error("A memory error occured")]
    MemoryError(ProcMemError),
}

impl From<ProcMemError> for ToggleCheatError {
    fn from(value: ProcMemError) -> Self {
        Self::MemoryError(value)
    }
}
