use super::{Entity, GameObject, Id};
use crate::Popcapgame;
use crate::il2cpp::{Array, Class, Element, ObjectError, Ref, class_layout, read_struct};

class_layout! {
    /// `Reloaded.Gameplay.DataArray<T>`, where the board keeps each kind of object.
    ///
    /// The objects are the first `m_count` items of `m_list`, with no gaps: freeing an object
    /// moves the last one into its place (`DataArray<T>.ReturnToPool`).
    pub struct DataArray<T> = "Reloaded.Gameplay.DataArray" {
        list: Ref<Array<DataArrayItem<T>>> = "m_list",
        count: i32 = "m_count",
    }
}

class_layout! {
    /// `DataArray<T>.DataArrayItem<T>`, a struct with an object and its ID.
    pub struct DataArrayItem<T> = "Reloaded.Gameplay.DataArray.DataArrayItem" {
        item: Option<Ref<T>> = "mItem",
    }
}

impl<T> Element for DataArrayItem<T> {
    fn read_element(game: &Popcapgame, class: Class, bytes: &[u8]) -> Result<Self, ObjectError> {
        read_struct(game, class, bytes)
    }
}

/// One of the kinds of objects that the board keeps a list of: plants, zombies, projectiles,
/// coins, lawn mowers and grid items.
pub trait BoardObject: GameObject {
    /// The object's ID, from the `DataID` that all of these classes inherit.
    fn id(fields: &Self::Fields) -> Id<Self>;

    /// Whether the object died. The board removes dead objects at the start of its next
    /// update (`Board.ProcessDeleteQueue`), so until then they're still in its list.
    fn is_dead(fields: &Self::Fields) -> bool;
}

impl<T: BoardObject> Entity<T> {
    /// The object's ID, which other objects use to refer to it, like a Bungee Zombie to the
    /// plant it grabbed.
    pub fn id(&self) -> Id<T> {
        T::id(&self.fields)
    }
}

/// Reads the objects in `array` that are alive, and set up.
pub(super) fn read_living<T: BoardObject>(
    game: &Popcapgame,
    array: Ref<DataArray<T::Fields>>,
) -> Result<Vec<Entity<T>>, ObjectError> {
    let array = array.read(game)?;
    let count = usize::try_from(array.count).unwrap_or_default();

    let mut living = Vec::with_capacity(count);
    for object in array.list.read_first(game, count)? {
        let Some(object) = object.item else {
            continue;
        };
        let fields = object.read(game)?;
        if T::is_dead(&fields) {
            continue;
        }
        // Objects that the game is still setting up aren't on the board yet.
        living.extend(Entity::new(object, fields));
    }
    Ok(living)
}
