use crate::game::Popcapgame;

pub trait WriteableEntity {
    fn write_entity(&self, addr: usize, game: &Popcapgame);
}
