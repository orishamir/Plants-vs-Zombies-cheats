pub struct ReaderAt {
    inner: Vec<u8>,
}

impl ReaderAt {
    pub fn new(inner: Vec<u8>) -> Self {
        Self { inner }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn read(&self, offset: impl Into<usize>, count: usize) -> Option<&[u8]> {
        let o: usize = offset.into();
        self.inner.get(o..o + count)
    }

    pub fn read_u32(&self, offset: impl Into<usize>) -> Option<u32> {
        let bytes = self.read(offset, 4)?;
        Some(u32::from_le_bytes(bytes.try_into().ok()?))
    }

    pub fn read_u8(&self, offset: impl Into<usize>) -> Option<u8> {
        let byte = self.read(offset, 1)?;
        Some(byte[0])
    }

    pub fn read_bool(&self, offset: impl Into<usize>) -> Option<bool> {
        let byte = self.read(offset, 1)?;
        Some(byte[0] != 0)
    }

    pub fn read_i32(&self, offset: impl Into<usize>) -> Option<i32> {
        let bytes = self.read(offset, 4)?;
        Some(i32::from_le_bytes(bytes.try_into().ok()?))
    }

    pub fn read_f32(&self, offset: impl Into<usize>) -> Option<f32> {
        let bytes = self.read(offset, 4)?;
        Some(f32::from_le_bytes(bytes.try_into().ok()?))
    }
}
