use std::array::TryFromSliceError;
use thiserror::Error;

pub struct ReaderAt {
    inner: Vec<u8>,
}

#[derive(Debug, Error)]
pub enum ReaderError {
    #[error("tried reading bytes but reached end of buffer")]
    ReadOutOfBounds,

    #[error("internal error: try from slice failed {0}")]
    SliceToArrayFailed(#[from] TryFromSliceError),
}

impl ReaderAt {
    pub fn new(inner: Vec<u8>) -> Self {
        Self { inner }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn read(&self, offset: impl Into<usize>, count: usize) -> Result<&[u8], ReaderError> {
        let offset: usize = offset.into();
        self.inner
            .get(offset..offset + count)
            .ok_or(ReaderError::ReadOutOfBounds)
    }

    pub fn read_u32(&self, offset: impl Into<usize>) -> Result<u32, ReaderError> {
        let bytes = self.read(offset, 4)?;
        Ok(u32::from_le_bytes(bytes.try_into()?))
    }

    pub fn read_bool(&self, offset: impl Into<usize>) -> Result<bool, ReaderError> {
        let byte = self.read(offset, 1)?;
        Ok(byte[0] != 0)
    }

    pub fn read_i32(&self, offset: impl Into<usize>) -> Result<i32, ReaderError> {
        let bytes = self.read(offset, 4)?;
        Ok(i32::from_le_bytes(bytes.try_into()?))
    }

    pub fn read_f32(&self, offset: impl Into<usize>) -> Result<f32, ReaderError> {
        let bytes = self.read(offset, 4)?;
        Ok(f32::from_le_bytes(bytes.try_into()?))
    }
}
