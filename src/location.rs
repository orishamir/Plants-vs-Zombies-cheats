use std::fmt;

use crate::{Game, MemoryError};

/// The game's DLLs that cheats change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dll {
    /// The game's own code, compiled from C# by IL2CPP.
    GameAssembly,
    /// The Unity engine.
    UnityPlayer,
}

impl Dll {
    /// The location `offset` bytes into this DLL, like Cheat Engine's `GameAssembly.dll+665DE0`.
    pub const fn at(self, offset: usize) -> Location {
        Location::InDll { dll: self, offset }
    }

    pub fn file_name(self) -> &'static str {
        match self {
            Dll::GameAssembly => "GameAssembly.dll",
            Dll::UnityPlayer => "UnityPlayer.dll",
        }
    }
}

/// Where something is in the game's memory.
///
/// A DLL can be loaded at a different address every time the game starts, so locations are
/// relative to a DLL and only become addresses in a running game.
#[derive(Debug, Clone, Copy)]
pub enum Location {
    /// `offset` bytes into a DLL, where its code and global variables are.
    /// Cheat Engine writes this as `GameAssembly.dll+665DE0`.
    InDll { dll: Dll, offset: usize },
    /// `offset` bytes into an object whose address is stored `pointer` bytes into a DLL.
    /// Cheat Engine writes this as `[UnityPlayer.dll+1F15830]+24F`.
    ThroughPointer {
        dll: Dll,
        pointer: usize,
        offset: usize,
    },
}

impl Location {
    /// The address this location is at in the running game.
    pub(crate) fn resolve(self, game: &Game) -> Result<usize, MemoryError> {
        match self {
            Location::InDll { dll, offset } => Ok(game.dll_base(dll) + offset),
            Location::ThroughPointer {
                dll,
                pointer,
                offset,
            } => Ok(game.read_pointer(game.dll_base(dll) + pointer)? + offset),
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Location::InDll { dll, offset } => write!(f, "{}+{offset:X}", dll.file_name()),
            Location::ThroughPointer {
                dll,
                pointer,
                offset,
            } => write!(f, "[{}+{pointer:X}]+{offset:X}", dll.file_name()),
        }
    }
}
