use super::{Instance, Object, ObjectError};
use crate::Game;

/// The fields this crate reads from one of the game's C# classes or structs, each with the
/// Rust type it's read as. Declare one with `class_layout!`.
pub trait Layout: Sized {
    /// The class's namespace and name, like `Reloaded.Gameplay.Plant`. For a generic class,
    /// leave out its type parameters: `System.Collections.Generic.List`.
    const CLASS: &'static str;

    /// Reads the fields from a copy of the object or struct.
    fn read(game: &Game, instance: &Instance) -> Result<Self, ObjectError>;

    /// Writes the fields whose values differ from `old`'s into `object`.
    fn write_changes(&self, old: &Self, game: &Game, object: Object) -> Result<(), ObjectError>;
}

/// Declares a [`Layout`]: a struct of the fields to read from one of the game's classes.
///
/// Each field has the Rust type to read it as (a [`FieldValue`](super::FieldValue)) and the
/// field's C# name, as Il2CppDumper's `dump.cs` shows it:
///
/// ```ignore
/// class_layout! {
///     /// `Reloaded.Gameplay.Coin`.
///     pub struct CoinFields = "Reloaded.Gameplay.Coin" {
///         x: f32 = "mPosX",
///         dead: bool = "mDead",
///     }
/// }
/// ```
///
/// Fields can come from the classes it inherits from too, like `mRow` from `ReloadedObject`.
/// A layout can have one type parameter, for generic classes like `List<T>`.
macro_rules! class_layout {
    (
        $(#[$attribute:meta])*
        $visibility:vis struct $name:ident $(<$parameter:ident>)? = $class:literal {
            $(
                $(#[$field_attribute:meta])*
                $field:ident: $type:ty = $cs_name:literal
            ),* $(,)?
        }
    ) => {
        $(#[$attribute])*
        #[derive(Debug, Clone, PartialEq)]
        $visibility struct $name $(<$parameter>)? {
            $(
                $(#[$field_attribute])*
                pub $field: $type,
            )*
        }

        impl $(<$parameter>)? $crate::il2cpp::Layout for $name $(<$parameter>)? {
            const CLASS: &'static str = $class;

            fn read(
                game: &$crate::Game,
                instance: &$crate::il2cpp::Instance,
            ) -> Result<Self, $crate::il2cpp::ObjectError> {
                Ok(Self {
                    $($field: instance.get(game, $cs_name)?,)*
                })
            }

            fn write_changes(
                &self,
                old: &Self,
                game: &$crate::Game,
                object: $crate::il2cpp::Object,
            ) -> Result<(), $crate::il2cpp::ObjectError> {
                $(
                    if self.$field != old.$field {
                        object.set(game, $cs_name, &self.$field)?;
                    }
                )*
                Ok(())
            }
        }
    };
}

pub(crate) use class_layout;
