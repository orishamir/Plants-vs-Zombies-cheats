use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use super::{Class, FieldValue, Layout, ObjectError, runtime};
use crate::Game;

/// One of the game's C# objects, like a plant or the board.
///
/// It's only where the object is: reading it gets the values its fields have right then.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Object(usize);

impl Object {
    /// The object at `address`, or `None` for a null reference.
    pub fn at(address: usize) -> Option<Self> {
        (address != 0).then_some(Self(address))
    }

    pub fn address(self) -> usize {
        self.0
    }

    pub fn class(self, game: &Game) -> Result<Class, ObjectError> {
        Ok(Class::at(
            game.read_pointer(self.0 + runtime::object::CLASS)?,
        ))
    }

    /// Copies the whole object, to read its fields from.
    pub fn read(self, game: &Game) -> Result<Instance, ObjectError> {
        let class = self.class(game)?;
        let size = class.info(game)?.instance_size;
        Ok(Instance {
            class,
            bytes: game.read_memory(self.0, size)?,
            header: 0,
        })
    }

    /// Reads one field.
    pub fn get<T: FieldValue>(self, game: &Game, name: &'static str) -> Result<T, ObjectError> {
        let class = self.class(game)?;
        let field = class.instance_field::<T>(game, name)?;
        let bytes = game.read_memory(self.0 + field.offset, T::SIZE)?;
        T::decode(&bytes).map_err(|error| class.invalid_value(game, name, error))
    }

    /// Writes one field.
    pub fn set<T: FieldValue>(
        self,
        game: &Game,
        name: &'static str,
        value: &T,
    ) -> Result<(), ObjectError> {
        let field = self.class(game)?.instance_field::<T>(game, name)?;
        Ok(game.write_memory(self.0 + field.offset, &value.encode())?)
    }
}

/// A copy of one of the game's objects or structs, to read its fields from.
#[derive(Debug)]
pub struct Instance {
    class: Class,
    bytes: Vec<u8>,
    /// How many bytes at the start of the object aren't in `bytes`. Field offsets count from
    /// the start of an object, but a struct inside an array or another object doesn't have
    /// the object header that starts every object.
    header: usize,
}

impl Instance {
    /// A struct stored inside an array, like an element of a `List<ModelReference>`.
    pub fn of_struct(class: Class, bytes: &[u8]) -> Self {
        Self {
            class,
            bytes: bytes.to_vec(),
            header: runtime::object::HEADER_SIZE,
        }
    }

    pub fn class(&self) -> Class {
        self.class
    }

    /// Reads one field from the copy.
    pub fn get<T: FieldValue>(&self, game: &Game, name: &'static str) -> Result<T, ObjectError> {
        let field = self.class.instance_field::<T>(game, name)?;
        let bytes = field
            .offset
            .checked_sub(self.header)
            .and_then(|start| self.bytes.get(start..start + T::SIZE))
            .ok_or_else(|| ObjectError::FieldOutsideObject {
                class: self.class.describe(game),
                field: name,
            })?;
        T::decode(bytes).map_err(|error| self.class.invalid_value(game, name, error))
    }
}

/// A reference to one of the game's objects, whose fields the [`Layout`] `T` lists.
///
/// It's a C# reference with the class it points to: `Ref<PlantFields>` is C#'s `Plant`.
pub struct Ref<T> {
    object: Object,
    layout: PhantomData<fn() -> T>,
}

impl<T> Ref<T> {
    pub fn object(self) -> Object {
        self.object
    }
}

impl<T: Layout> Ref<T> {
    /// Reads the object's fields. Fails if the object isn't a `T::CLASS`.
    pub fn read(self, game: &Game) -> Result<T, ObjectError> {
        let instance = self.object.read(game)?;
        if !instance.class().is_a(game, T::CLASS)? {
            return Err(ObjectError::WrongClass {
                address: self.object.address(),
                expected: T::CLASS,
                found: instance.class().describe(game),
            });
        }
        T::read(game, &instance)
    }
}

impl<T> From<Object> for Ref<T> {
    fn from(object: Object) -> Self {
        Self {
            object,
            layout: PhantomData,
        }
    }
}

// Written out instead of derived, because deriving them would require `T` to have them too.

impl<T> Clone for Ref<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Ref<T> {}

impl<T> PartialEq for Ref<T> {
    fn eq(&self, other: &Self) -> bool {
        self.object == other.object
    }
}

impl<T> Eq for Ref<T> {}

impl<T> Hash for Ref<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.object.hash(state);
    }
}

impl<T> fmt::Debug for Ref<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ref({:#X})", self.object.address())
    }
}
