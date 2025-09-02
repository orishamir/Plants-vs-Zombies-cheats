use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
use proc_mem::ProcMemError;
use thiserror::Error;

use crate::{Popcapgame, readers::offset_reader::OffsetReaderError};

pub trait ReadableEntity: Sized {
    fn read(game: &Popcapgame) -> Result<Self, ReadEntityError>;
}

#[derive(Debug, Error)]
pub enum MemoryReaderError {
    #[error("internal error: failed while converting from vector to array")]
    FailedConvertion,
    #[error("tried reading bytes but reached end of buffer")]
    ReadOutOfBounds,
}

#[derive(Debug, Error)]
pub enum ReadEntityError {
    #[error("Unknown integer value for enum: {0}")]
    /// for example an unknown PlantType encountered
    UnknownEnumMember(String),

    #[error("A memory error occured: {0:?}")]
    ProcMemError(ProcMemError),

    #[error("A memory error occured: {0:?}")]
    MemoryReaderError(#[from] MemoryReaderError),

    #[error("An offset readererror occured: {0:?}")]
    OffsetReaderError(#[from] OffsetReaderError),
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
