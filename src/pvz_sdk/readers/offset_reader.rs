use thiserror::Error;

use crate::Popcapgame;

pub struct OffsetReader<'a> {
    pub base_addr: usize,
    game: &'a Popcapgame,
}

#[derive(Debug, Error)]
pub enum OffsetReaderError {
    #[error("tried reading bytes but reached end of buffer")]
    ReadOutOfBounds,

    #[error("internal error: try from slice failed")]
    SliceToArrayFailed,
}

impl<'a> OffsetReader<'a> {
    pub fn new(base_addr: usize, game: &'a Popcapgame) -> Self {
        Self { base_addr, game }
    }

    pub fn read(
        &self,
        offset: impl Into<usize>,
        count: usize,
    ) -> Result<Vec<u8>, OffsetReaderError> {
        let offset: usize = offset.into();
        self.game
            .read_bytes_at(self.base_addr + offset, count)
            .ok_or(OffsetReaderError::ReadOutOfBounds)
    }

    pub fn seek_by(&mut self, amount: usize) {
        self.base_addr += amount;
    }

    pub fn read_u32(&self, offset: impl Into<usize>) -> Result<u32, OffsetReaderError> {
        let bytes = self.read(offset, 4)?;
        Ok(u32::from_le_bytes(
            bytes
                .try_into()
                .map_err(|_| OffsetReaderError::SliceToArrayFailed)?,
        ))
    }

    pub fn read_usize(&self, offset: impl Into<usize>) -> Result<usize, OffsetReaderError> {
        let bytes = self.read(offset, 4)?;
        Ok(usize::from_le_bytes(
            bytes
                .try_into()
                .map_err(|_| OffsetReaderError::SliceToArrayFailed)?,
        ))
    }

    pub fn read_bool(&self, offset: impl Into<usize>) -> Result<bool, OffsetReaderError> {
        let byte = self.read(offset, 1)?;
        Ok(byte[0] != 0)
    }

    pub fn read_i32(&self, offset: impl Into<usize>) -> Result<i32, OffsetReaderError> {
        let bytes = self.read(offset, 4)?;
        Ok(i32::from_le_bytes(
            bytes
                .try_into()
                .map_err(|_| OffsetReaderError::SliceToArrayFailed)?,
        ))
    }

    pub fn read_f32(&self, offset: impl Into<usize>) -> Result<f32, OffsetReaderError> {
        let bytes = self.read(offset, 4)?;
        Ok(f32::from_le_bytes(
            bytes
                .try_into()
                .map_err(|_| OffsetReaderError::SliceToArrayFailed)?,
        ))
    }
}
