use crate::game::Popcapgame;

pub trait ReadableEntity {
    const SIZE: usize;
    /// You will get a buffer the size of your struct
    fn from_bytes(buf: &[u8]) -> Self;
}

#[allow(dead_code)]
pub trait WriteableEntity {
    fn write_entity(&self, addr: usize, game: &Popcapgame);
}
