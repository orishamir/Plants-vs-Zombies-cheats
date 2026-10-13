//! The running game: attaching to it, and reading and writing its memory.

use thiserror::Error;

use crate::Dll;
use crate::il2cpp::ClassCache;
use crate::process::{AttachError, Process};

/// Reading or writing the game's memory failed, for example because the game closed.
#[derive(Debug, Error)]
pub enum MemoryError {
    #[error("couldn't read {len} bytes of the game's memory at {address:#X}")]
    Read {
        address: usize,
        len: usize,
        source: windows::core::Error,
    },

    #[error("couldn't write {len} bytes of the game's memory at {address:#X}")]
    Write {
        address: usize,
        len: usize,
        source: windows::core::Error,
    },
}

/// A number the game stores in its memory as little-endian bytes, like an `i32` or a pointer.
pub trait Primitive: Copy {
    /// Decodes it from exactly `size_of::<Self>()` bytes.
    ///
    /// # Panics
    ///
    /// If `bytes` is a different length.
    fn from_le_slice(bytes: &[u8]) -> Self;

    fn to_le_vec(self) -> Vec<u8>;

    /// Decodes it from the bytes at `offset` in `bytes`, like a field from a copy of a struct.
    ///
    /// # Panics
    ///
    /// If `bytes` ends before the number does.
    fn from_le_slice_at(bytes: &[u8], offset: usize) -> Self {
        Self::from_le_slice(&bytes[offset..][..size_of::<Self>()])
    }
}

macro_rules! impl_primitive {
    ($($type:ty),*) => {$(
        impl Primitive for $type {
            fn from_le_slice(bytes: &[u8]) -> Self {
                let bytes = bytes.try_into().unwrap_or_else(|_| {
                    panic!("{} takes {} bytes, not {}", stringify!($type), size_of::<$type>(), bytes.len())
                });
                <$type>::from_le_bytes(bytes)
            }

            fn to_le_vec(self) -> Vec<u8> {
                <$type>::to_le_bytes(self).to_vec()
            }
        }
    )*};
}

impl_primitive!(u8, u16, u32, u64, usize, i8, i16, i32, i64, f32, f64);

/// The running game, and what's been learned about its classes so far.
///
/// Outside this crate, it's what [`Board::current`](crate::entities::Board::current) and the
/// cheats read from and write to. Reading and writing at addresses stays inside the crate.
#[derive(Debug)]
pub struct Game {
    process: Process,
    /// Where `GameAssembly.dll` is loaded in the game.
    game_assembly: usize,
    /// Where `UnityPlayer.dll` is loaded in the game.
    unity_player: usize,
    classes: ClassCache,
}

impl Game {
    /// Attaches to the running game.
    pub fn attach() -> Result<Self, AttachError> {
        let process = Process::open("Replanted.exe")?;
        Ok(Self {
            game_assembly: process.module_base(Dll::GameAssembly.file_name())?,
            unity_player: process.module_base(Dll::UnityPlayer.file_name())?,
            process,
            classes: ClassCache::default(),
        })
    }

    /// The address `dll` is loaded at in the game.
    pub(crate) fn dll_base(&self, dll: Dll) -> usize {
        match dll {
            Dll::GameAssembly => self.game_assembly,
            Dll::UnityPlayer => self.unity_player,
        }
    }

    /// Reads `len` bytes at `address`.
    pub(crate) fn read_memory(&self, address: usize, len: usize) -> Result<Vec<u8>, MemoryError> {
        let mut buffer = vec![0; len];
        self.process
            .read(address, &mut buffer)
            .map(|()| buffer)
            .map_err(|source| MemoryError::Read {
                address,
                len,
                source,
            })
    }

    /// Reads the number at `address`.
    pub(crate) fn read<T: Primitive>(&self, address: usize) -> Result<T, MemoryError> {
        Ok(T::from_le_slice(
            &self.read_memory(address, size_of::<T>())?,
        ))
    }

    /// Reads the pointer stored at `address`.
    pub(crate) fn read_pointer(&self, address: usize) -> Result<usize, MemoryError> {
        self.read(address)
    }

    /// Writes `bytes` at `address`.
    pub(crate) fn write_memory(&self, address: usize, bytes: &[u8]) -> Result<(), MemoryError> {
        self.process
            .write(address, bytes)
            .map_err(|source| MemoryError::Write {
                address,
                len: bytes.len(),
                source,
            })
    }

    /// What's been read about the game's classes, so that each is only read once.
    pub(crate) fn classes(&self) -> &ClassCache {
        &self.classes
    }
}
