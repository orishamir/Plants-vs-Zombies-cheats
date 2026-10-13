//! Reading and writing the game's C# objects, finding their fields by name.
//!
//! The game is C# compiled to native code by Unity's IL2CPP. Like C#'s reflection, IL2CPP
//! keeps every class's name and fields in the game's memory, with where each field is in an
//! object. This module reads that, so the rest of the code names fields the way the game's
//! code does, like `Plant.mPlantHealth`, instead of by offsets that change whenever a game
//! update adds a field. Il2CppDumper's `dump.cs` lists every class and field (see
//! `tools/re-tools/README.md`).
//!
//! - [`Class`] and [`Object`] are one of the game's classes and objects.
//! - A [`Layout`], declared with `class_layout!`, lists the fields to read from a class and
//!   the Rust type to read each one as, a [`FieldValue`].
//! - [`Ref`] is a reference to an object whose fields a [`Layout`] lists, and [`Array`] and
//!   [`List`] are C#'s collections.
//!
//! The only offsets left are in IL2CPP's own structs (`runtime.rs`), which only change when
//! the game moves to a new version of Unity.

mod class;
mod collections;
mod layout;
mod object;
mod runtime;
mod value;

use thiserror::Error;

use crate::MemoryError;

pub(crate) use class::ClassCache;
pub use class::{Class, TypeInfo};
pub use collections::{Array, CsString, Element, List, read_struct};
pub use layout::Layout;
pub(crate) use layout::class_layout;
pub use object::{Instance, Object, Ref};
pub use value::{DecodeError, Enum, EnumWithNone, FieldValue, TypeKind};

/// Reading or writing one of the game's objects failed.
#[derive(Debug, Error)]
pub enum ObjectError {
    #[error(transparent)]
    Memory(#[from] MemoryError),

    /// A class doesn't have a field this crate uses, usually because a game update renamed it.
    #[error("{class} has no field named {field}. The game was probably updated")]
    MissingField { class: String, field: &'static str },

    /// A field's type isn't the one this crate reads it as.
    #[error("{class}.{field} is a {found:?}, which can't be read as a {expected}")]
    WrongFieldType {
        class: String,
        field: &'static str,
        found: TypeKind,
        expected: &'static str,
    },

    /// An array's elements aren't of the type this crate reads them as.
    #[error("a {class}[]'s elements are {found:?}s, which can't be read as {expected}")]
    WrongElementType {
        class: String,
        found: TypeKind,
        expected: &'static str,
    },

    /// A field's value isn't one its Rust type can hold, like an enum value this crate doesn't
    /// know.
    #[error("{place} can't be read: {error}")]
    InvalidValue { place: String, error: DecodeError },

    /// An object isn't of the class this crate expected, which means the reference to it is
    /// out of date or wrong.
    #[error("expected a {expected} at {address:#X}, but found a {found}")]
    WrongClass {
        address: usize,
        expected: &'static str,
        found: String,
    },

    /// The game is still setting up an object, so some of its fields are empty.
    #[error("the game hasn't finished setting up this {class} yet")]
    Unfinished { class: &'static str },

    /// IL2CPP only sets up a class the first time the game uses it.
    #[error("{type_info} isn't set, so the game hasn't used that class yet")]
    ClassNotLoaded { type_info: &'static str },

    #[error("{class}'s fields aren't loaded yet")]
    FieldsNotLoaded { class: String },

    #[error("{class}.{field} isn't inside the object, so the class was read wrong")]
    FieldOutsideObject { class: String, field: &'static str },

    #[error("{class} isn't an array")]
    NotAnArray { class: String },

    #[error("the name at {address:#X} isn't valid UTF-8")]
    InvalidName { address: usize },
}
