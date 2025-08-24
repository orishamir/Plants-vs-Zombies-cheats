pub trait MemoryParseable {
    /// You will get a buffer the size of your struct
    fn from_bytes(buf: &[u8]) -> Self;
}
