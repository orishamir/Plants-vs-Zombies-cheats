use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::num::NonZeroU32;

use crate::game::Primitive;
use crate::il2cpp::{DecodeError, FieldValue, TypeKind};

/// Which of the board's objects of type `T` something is, like the plant a Bungee Zombie
/// grabbed. The game refers to objects this way, instead of by where they are in memory.
///
/// Every object on the board gets an ID when the game creates it (`ReloadedObject.DataID`),
/// from a counter that only goes up (`DataArray.mNextKey`). To find the object an ID names,
/// look for it among the objects of its kind:
///
/// ```no_run
/// # use pvz_sdk::{Popcapgame, entities::Board};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let game = Popcapgame::init()?;
/// # let board = Board::current(&game)?.expect("in a level");
/// let plants = board.plants(&game)?;
/// for zombie in board.zombies(&game)? {
///     if let Some(id) = zombie.grabbed_plant {
///         let plant = plants.iter().find(|plant| plant.id() == id);
///         println!("{:?} grabbed {:?}", zombie.zombie_type, plant.map(|p| p.plant_type));
///     }
/// }
/// # Ok(())
/// # }
/// ```
pub struct Id<T> {
    value: NonZeroU32,
    kind: PhantomData<fn() -> T>,
}

impl<T> Id<T> {
    pub(super) const fn new(value: NonZeroU32) -> Self {
        Self {
            value,
            kind: PhantomData,
        }
    }
}

/// An ID field, like `Zombie.mTargetPlantID`. The game's IDs (`DataID`, `PlantID`...) are
/// structs with one 32-bit number, and 0 means no object.
impl<T> FieldValue for Option<Id<T>> {
    const SIZE: usize = size_of::<u32>();

    fn fits(kind: TypeKind) -> bool {
        kind == TypeKind::ValueType
    }

    fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Ok(NonZeroU32::new(u32::from_le_slice(bytes)).map(Id::new))
    }

    fn encode(&self) -> Vec<u8> {
        self.map_or(0, |id| id.value.get()).to_le_vec()
    }
}

impl<T> FieldValue for Id<T> {
    const SIZE: usize = size_of::<u32>();

    fn fits(kind: TypeKind) -> bool {
        Option::<Self>::fits(kind)
    }

    fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Option::<Self>::decode(bytes)?.ok_or(DecodeError::Null)
    }

    fn encode(&self) -> Vec<u8> {
        Some(*self).encode()
    }
}

// Written out instead of derived, because deriving them would require `T` to have them too.

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Id<T> {}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> Eq for Id<T> {}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T> fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Id({})", self.value)
    }
}

/// An ID to fill in the fields of test objects.
#[cfg(test)]
pub(super) fn test_id<T>(value: u32) -> Id<T> {
    Id::new(NonZeroU32::new(value).expect("IDs aren't 0"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_is_no_object() {
        assert_eq!(Option::<Id<()>>::decode(&[0; 4]).unwrap(), None);
        assert_eq!(
            Option::<Id<()>>::decode(&7_u32.to_le_bytes()).unwrap(),
            Some(test_id(7))
        );
        assert!(matches!(Id::<()>::decode(&[0; 4]), Err(DecodeError::Null)));
        assert_eq!(Option::<Id<()>>::None.encode(), [0; 4]);
    }
}
