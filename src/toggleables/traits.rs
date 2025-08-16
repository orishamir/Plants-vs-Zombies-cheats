use crate::game::GameProcess;

pub trait Toggleable {
    fn activate(&self, process: &GameProcess) -> Result<(), ToggleCheatError>;
    fn deactivate(&self, process: &GameProcess) -> Result<(), ToggleCheatError>;
    fn get_name(&self) -> &'static str;
    fn toggle(&mut self, process: &GameProcess, toggled: bool) -> Result<(), ToggleCheatError> {
        if toggled {
            self.activate(process)
        } else {
            self.deactivate(process)
        }
    }
}

#[derive(Debug)]
pub enum ToggleCheatError {
    IoError(std::io::Error),
}

impl From<std::io::Error> for ToggleCheatError {
    fn from(value: std::io::Error) -> Self {
        ToggleCheatError::IoError(value)
    }
}
