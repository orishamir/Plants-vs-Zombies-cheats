use std::marker::PhantomData;

use super::{Class, FieldValue, Instance, Layout, ObjectError, Ref, class_layout, runtime};
use crate::Popcapgame;
use crate::game::Primitive;

/// A C# array, `E[]`. Read one through a `Ref<Array<E>>`.
pub struct Array<E> {
    element: PhantomData<fn() -> E>,
}

/// Something an array can hold: a value like an `i32`, a reference, or a struct.
pub trait Element: Sized {
    /// Reads one element from its bytes in the array.
    fn read_element(game: &Popcapgame, class: Class, bytes: &[u8]) -> Result<Self, ObjectError>;
}

impl<T: FieldValue> Element for T {
    fn read_element(game: &Popcapgame, class: Class, bytes: &[u8]) -> Result<Self, ObjectError> {
        let kind = class.info(game)?.kind;
        if !T::fits(kind) {
            return Err(ObjectError::WrongElementType {
                class: class.describe(game),
                found: kind,
                expected: std::any::type_name::<T>(),
            });
        }
        T::decode(&bytes[..T::SIZE]).map_err(|error| ObjectError::InvalidValue {
            place: format!("an element of a {}[]", class.describe(game)),
            error,
        })
    }
}

/// Reads a struct stored in an array, for implementing [`Element`] for a struct's [`Layout`].
pub fn read_struct<T: Layout>(
    game: &Popcapgame,
    class: Class,
    bytes: &[u8],
) -> Result<T, ObjectError> {
    T::read(game, &Instance::of_struct(class, bytes))
}

impl<E> Ref<Array<E>> {
    /// How many elements the array has.
    pub fn len(self, game: &Popcapgame) -> Result<usize, ObjectError> {
        Ok(game.read(self.object().address() + runtime::array::LENGTH)?)
    }
}

impl<E: Element> Ref<Array<E>> {
    pub fn read_all(self, game: &Popcapgame) -> Result<Vec<E>, ObjectError> {
        self.read_first(game, usize::MAX)
    }

    /// Reads the first `count` elements, or all of them if there are fewer.
    pub fn read_first(self, game: &Popcapgame, count: usize) -> Result<Vec<E>, ObjectError> {
        let class = self.object().class(game)?;
        let (element_class, element_size) =
            class
                .info(game)?
                .element
                .ok_or_else(|| ObjectError::NotAnArray {
                    class: class.describe(game),
                })?;

        let len = self.len(game)?.min(count);
        if len == 0 {
            return Ok(Vec::new());
        }
        let elements = game.read_memory(
            self.object().address() + runtime::array::ELEMENTS,
            len * element_size,
        )?;
        elements
            .chunks_exact(element_size)
            .map(|bytes| E::read_element(game, element_class, bytes))
            .collect()
    }
}

/// A C# two-dimensional array, `E[,]`. Read one through a `Ref<Array2D<E>>`.
pub struct Array2D<E> {
    element: PhantomData<fn() -> E>,
}

impl<E: Element> Ref<Array2D<E>> {
    /// Reads every element, as rows: `rows[i][j]` is the C# array's `[i, j]`.
    pub fn read_all(self, game: &Popcapgame) -> Result<Vec<Vec<E>>, ObjectError> {
        let address = self.object().address();
        let class = self.object().class(game)?;
        let info = class.info(game)?;
        let (element_class, element_size) =
            info.element.ok_or_else(|| ObjectError::NotAnArray {
                class: class.describe(game),
            })?;
        if info.rank != 2 {
            return Err(ObjectError::WrongDimensions {
                class: class.describe(game),
                expected: 2,
                found: info.rank,
            });
        }

        let bounds = game.read_memory(
            game.read_pointer(address + runtime::array::BOUNDS)?,
            2 * runtime::array_bounds::SIZE,
        )?;
        let length_of = |dimension: usize| {
            usize::from_le_slice_at(
                &bounds,
                dimension * runtime::array_bounds::SIZE + runtime::array_bounds::LENGTH,
            )
        };
        let (rows, columns) = (length_of(0), length_of(1));
        if rows * columns == 0 {
            return Ok((0..rows).map(|_| Vec::new()).collect());
        }

        let bytes = game.read_memory(
            address + runtime::array::ELEMENTS,
            rows * columns * element_size,
        )?;
        let mut elements = bytes
            .chunks_exact(element_size)
            .map(|bytes| E::read_element(game, element_class, bytes))
            .collect::<Result<Vec<E>, _>>()?
            .into_iter();
        Ok((0..rows)
            .map(|_| elements.by_ref().take(columns).collect())
            .collect())
    }
}

class_layout! {
    /// C#'s `List<T>`: an array that's usually bigger than the list, and how much of it the
    /// list uses.
    pub struct List<E> = "System.Collections.Generic.List" {
        items: Ref<Array<E>> = "_items",
        size: i32 = "_size",
    }
}

impl<E: Element> List<E> {
    pub fn elements(&self, game: &Popcapgame) -> Result<Vec<E>, ObjectError> {
        let size = usize::try_from(self.size).unwrap_or_default();
        self.items.read_first(game, size)
    }
}

/// A C# `string`. Read one through a `Ref<CsString>`.
pub struct CsString {
    _private: (),
}

impl Ref<CsString> {
    pub fn read_string(self, game: &Popcapgame) -> Result<String, ObjectError> {
        let address = self.object().address();
        let len: i32 = game.read(address + runtime::string::LENGTH)?;
        let len = usize::try_from(len).unwrap_or_default();
        let bytes = game.read_memory(address + runtime::string::CHARS, len * 2)?;
        let units: Vec<u16> = bytes
            .as_chunks()
            .0
            .iter()
            .map(|&unit| u16::from_le_bytes(unit))
            .collect();
        Ok(String::from_utf16_lossy(&units))
    }
}
