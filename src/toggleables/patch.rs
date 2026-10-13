use super::{ToggleCheatError, Toggleable};
use crate::{Game, Location};

/// A change to the game's memory, usually its code: some bytes, and what to replace them with.
///
/// A patch only writes over the bytes it expects to find. When a game update moves the code,
/// it reports [`ToggleCheatError::UnexpectedBytes`] instead of overwriting whatever is there.
#[derive(Debug)]
pub struct Patch {
    /// Where the bytes are.
    pub location: Location,
    /// The game's own bytes, which turning the cheat off puts back.
    pub original: &'static [u8],
    /// The bytes the cheat writes instead. Always as many as `original`.
    pub patched: &'static [u8],
}

/// Which of its two versions a patch's location holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PatchState {
    Original,
    Patched,
}

impl Patch {
    fn state(&self, game: &Game) -> Result<PatchState, ToggleCheatError> {
        let address = self.location.resolve(game)?;
        let found = game.read_memory(address, self.original.len())?;
        if found == self.original {
            Ok(PatchState::Original)
        } else if found == self.patched {
            Ok(PatchState::Patched)
        } else {
            Err(ToggleCheatError::UnexpectedBytes {
                location: self.location,
                expected: self.original,
                found,
            })
        }
    }

    fn write(&self, game: &Game, state: PatchState) -> Result<(), ToggleCheatError> {
        let bytes = match state {
            PatchState::Original => self.original,
            PatchState::Patched => self.patched,
        };
        let address = self.location.resolve(game)?;
        Ok(game.write_memory(address, bytes)?)
    }
}

/// A cheat made of patches to the game's code. Turning it on writes every patch, and turning
/// it off puts the game's own bytes back.
#[derive(Debug)]
pub struct PatchCheat {
    pub name: &'static str,
    pub patches: &'static [Patch],
}

impl PatchCheat {
    /// Writes every patch's bytes for `state`. All the patches are checked first, so if any
    /// of them is out of date, nothing is written.
    fn set(&self, game: &Game, state: PatchState) -> Result<(), ToggleCheatError> {
        for patch in self.patches {
            patch.state(game)?;
        }
        self.patches
            .iter()
            .try_for_each(|patch| patch.write(game, state))
    }
}

impl Toggleable for PatchCheat {
    fn activate(&self, game: &Game) -> Result<(), ToggleCheatError> {
        self.set(game, PatchState::Patched)
    }

    fn deactivate(&self, game: &Game) -> Result<(), ToggleCheatError> {
        self.set(game, PatchState::Original)
    }

    /// The cheat is on only when every one of its patches is in place.
    fn is_activated(&self, game: &Game) -> Result<bool, ToggleCheatError> {
        let states = self
            .patches
            .iter()
            .map(|patch| patch.state(game))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(states.iter().all(|&state| state == PatchState::Patched))
    }

    fn name(&self) -> &'static str {
        self.name
    }
}
