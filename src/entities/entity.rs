use std::fmt;
use std::ops::{Deref, DerefMut};

use crate::Popcapgame;
use crate::il2cpp::{Layout, ObjectError, Ref};

/// A type that one of the game's classes is read as, like [`Plant`](super::Plant) for
/// `Reloaded.Gameplay.Plant`.
///
/// The class's fields are read into [`Self::Fields`], which has them as the game stores them,
/// and then turned into this type. To write changes back, this type puts its values back
/// into the fields, and only the fields that changed are written.
pub trait GameObject: Sized {
    /// The class's fields, as the game stores them.
    type Fields: Layout + Clone;

    /// The value the fields hold, or `None` if they don't make a whole one yet. The game
    /// reuses objects, and while it sets one up, some of its fields can still be empty, like
    /// the plant of a seed packet that it hasn't picked yet.
    fn from_fields(fields: &Self::Fields) -> Option<Self>;

    /// Puts this value's data into `fields`, leaving any fields it doesn't use as they are.
    fn update_fields(&self, fields: &mut Self::Fields);
}

/// One of the game's objects, as it was when it was read: a [`Plant`](super::Plant), a
/// [`Zombie`](super::Zombie), the [`Board`](super::Board)...
///
/// It's used like the value inside it. To change the object in the game, change the value
/// and [`save`](Self::save) it:
///
/// ```no_run
/// # use pvz_sdk::{Popcapgame, entities::Board};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let game = Popcapgame::init()?;
/// if let Some(board) = Board::current(&game)? {
///     for mut zombie in board.zombies(&game)? {
///         zombie.is_hypnotized = true;
///         zombie.save(&game)?;
///     }
/// }
/// # Ok(())
/// # }
/// ```
pub struct Entity<T: GameObject> {
    object: Ref<T::Fields>,
    /// The object's fields as they were read, or last saved.
    pub(super) fields: T::Fields,
    value: T,
}

impl<T: GameObject> Entity<T> {
    /// The entity of an object with these fields, or `None` if the game is still setting the
    /// object up (see [`GameObject::from_fields`]).
    pub(super) fn new(object: Ref<T::Fields>, fields: T::Fields) -> Option<Self> {
        Some(Self {
            object,
            value: T::from_fields(&fields)?,
            fields,
        })
    }

    pub(super) fn read(game: &Popcapgame, object: Ref<T::Fields>) -> Result<Self, ObjectError> {
        Self::new(object, object.read(game)?).ok_or(ObjectError::Unfinished {
            class: <T::Fields as Layout>::CLASS,
        })
    }

    /// Writes the changes made to the value into the game. Only the fields that changed are
    /// written, so this doesn't undo what the game changed since the object was read, unless
    /// it's in a field that was changed here too.
    pub fn save(&mut self, game: &Popcapgame) -> Result<(), ObjectError> {
        let mut fields = self.fields.clone();
        self.value.update_fields(&mut fields);
        fields.write_changes(&self.fields, game, self.object.object())?;
        self.fields = fields;
        Ok(())
    }

    /// Reads the object again, dropping any changes that weren't saved.
    pub fn refresh(&mut self, game: &Popcapgame) -> Result<(), ObjectError> {
        *self = Self::read(game, self.object)?;
        Ok(())
    }

    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T: GameObject> Deref for Entity<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T: GameObject> DerefMut for Entity<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T: GameObject + fmt::Debug> fmt::Debug for Entity<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl<T: GameObject + Clone> Clone for Entity<T> {
    fn clone(&self) -> Self {
        Self {
            object: self.object,
            fields: self.fields.clone(),
            value: self.value.clone(),
        }
    }
}

/// Checks that a [`GameObject`]'s [`update_fields`](GameObject::update_fields) writes back
/// everything [`from_fields`](GameObject::from_fields) reads.
///
/// `first` and `second` are the fields of two objects that differ in every field. Writing
/// `second`'s value into `first`'s fields must give `second`'s value back: anything
/// `update_fields` forgets would still be `first`'s.
#[cfg(test)]
pub(super) fn assert_round_trips<T>(first: T::Fields, second: T::Fields)
where
    T: GameObject + PartialEq + fmt::Debug,
{
    let read = |fields: &T::Fields| T::from_fields(fields).expect("the fields make a whole value");
    let second_value = read(&second);
    assert_ne!(read(&first), second_value, "test with two different values");

    let mut fields = first;
    second_value.update_fields(&mut fields);
    assert_eq!(read(&fields), second_value);
}
