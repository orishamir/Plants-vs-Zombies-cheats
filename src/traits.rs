use crate::{game::Popcapgame, parsers::reader_at::ReaderAt};

pub trait ReadableEntity {
    const SIZE: usize;
    /// You should get a buffer the size of your struct
    fn read(reader: ReaderAt) -> Self;
}

pub trait WriteableEntity {
    fn write_entity(&self, addr: usize, game: &Popcapgame);
}
