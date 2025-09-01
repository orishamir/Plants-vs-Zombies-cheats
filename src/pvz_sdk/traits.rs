use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
use proc_mem::ProcMemError;
use thiserror::Error;

use crate::{ReaderAt, game::Popcapgame, reader_at::ReaderError};

pub trait ReadableEntity: Sized {
    const SIZE: usize;
    /// You should get a buffer the size of your struct
    fn read(reader: ReaderAt) -> Result<Self, ReadEntityError>;
}

pub trait WriteableEntity {
    fn write_entity(&self, addr: usize, game: &Popcapgame);
}

#[derive(Debug, Error)]
pub enum ReadEntityError {
    #[error("Unknown integer value for enum: {0}")]
    /// for example an unknown PlantType encountered
    UnknownEnumMember(String),

    #[error("A memory error occured: {0:?}")]
    ProcMemError(ProcMemError),

    #[error("ReaderAt method failed: {0}")]
    ReaderError(#[from] ReaderError),
}

impl From<ProcMemError> for ReadEntityError {
    fn from(value: ProcMemError) -> Self {
        Self::ProcMemError(value)
    }
}

impl<T: TryFromPrimitive> From<TryFromPrimitiveError<T>> for ReadEntityError {
    fn from(e: TryFromPrimitiveError<T>) -> Self {
        Self::UnknownEnumMember(e.to_string())
    }
}
