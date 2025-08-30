use std::fmt::Debug;

use crate::{game::Popcapgame, traits::WriteableEntity};

#[derive(Debug)]
pub struct CheatedEntity<T: WriteableEntity> {
    addr: usize,
    pub entity: T,
}

impl<T: WriteableEntity> CheatedEntity<T> {
    pub fn new(addr: usize, entity: T) -> Self {
        Self { addr, entity }
    }

    pub fn write_entity(&self, game: &Popcapgame) {
        self.entity.write_entity(self.addr, game);
    }
}
