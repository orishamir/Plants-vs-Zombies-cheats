use crate::game::Popcapgame;

pub trait ReadableEntity {
    /// You will get a buffer the size of your struct
    fn from_bytes(buf: &Vec<u8>) -> Self;

    fn size_of() -> usize;
}

#[allow(dead_code)]
pub trait WriteableEntity {
    fn write_entity(&self, addr: usize, game: &Popcapgame);
}
