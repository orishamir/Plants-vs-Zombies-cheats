use crate::traits::ReadableEntity;

struct CheatedEntity<T: ReadableEntity> {
    addr: usize,
    pub entity: T,
}

impl<T: ReadableEntity> CheatedEntity<T> {}
