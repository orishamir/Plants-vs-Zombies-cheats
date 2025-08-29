use std::fmt::Debug;

use crate::{
    game::Popcapgame,
    traits::{ReadableEntity, WriteableEntity},
};

#[derive(Debug)]
pub struct CheatedEntity<T: ReadableEntity + WriteableEntity> {
    addr: usize,
    pub entity: T,
}

impl<T: ReadableEntity + WriteableEntity> CheatedEntity<T> {
    pub fn new(addr: usize, entity: T) -> Self {
        Self { addr, entity }
    }

    pub fn write_entity(&self, game: &Popcapgame) {
        self.entity.write_entity(self.addr, game);
    }
}
