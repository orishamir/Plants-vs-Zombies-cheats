pub trait MemoryParseable {
    /// You will get a buffer the size of your struct
    fn from_bytes(buf: Vec<u8>) -> Self;

    fn size_of() -> usize;
}
