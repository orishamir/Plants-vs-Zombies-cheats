use num_enum::FromPrimitive;
use thiserror::Error;

use super::{Object, Ref};
use crate::game::Primitive;

/// What kind of type a C# field has, as IL2CPP records it (its `Il2CppTypeEnum`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u8)]
pub enum TypeKind {
    Bool = 0x02,
    Char = 0x03,
    SByte = 0x04,
    Byte = 0x05,
    Short = 0x06,
    UShort = 0x07,
    Int = 0x08,
    UInt = 0x09,
    Long = 0x0A,
    ULong = 0x0B,
    Float = 0x0C,
    Double = 0x0D,
    String = 0x0E,
    Pointer = 0x0F,
    /// A struct or an enum.
    ValueType = 0x11,
    Class = 0x12,
    /// A type parameter of a generic class, like `T` in `List<T>`.
    GenericParameter = 0x13,
    /// A multidimensional array, like `int[,]`.
    MultiArray = 0x14,
    /// A generic class or struct with its type arguments, like `List<Plant>`.
    GenericInstance = 0x15,
    IntPtr = 0x18,
    UIntPtr = 0x19,
    Object = 0x1C,
    /// A one-dimensional array, like `int[]`.
    Array = 0x1D,
    #[num_enum(catch_all)]
    Other(u8),
}

impl TypeKind {
    /// Whether a field of this kind holds a reference to an object.
    pub fn is_reference(self) -> bool {
        matches!(
            self,
            Self::Class
                | Self::GenericInstance
                | Self::String
                | Self::Object
                | Self::Array
                | Self::MultiArray
        )
    }
}

/// Why a field's bytes can't be read as the value they're supposed to be.
#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("{value} isn't a known {type_name}")]
    UnknownValue { value: i32, type_name: &'static str },

    #[error("it's null")]
    Null,
}

/// A Rust type that a C# field can be read as.
pub trait FieldValue: Sized {
    /// How many bytes the value takes in an object.
    const SIZE: usize;

    /// Whether a C# field of this kind can be read as this type. Reading a field checks this,
    /// so that reading the wrong field fails instead of returning nonsense.
    fn fits(kind: TypeKind) -> bool;

    /// Decodes the value from exactly [`Self::SIZE`] bytes.
    fn decode(bytes: &[u8]) -> Result<Self, DecodeError>;

    fn encode(&self) -> Vec<u8>;
}

macro_rules! primitive_field_values {
    ($($type:ty => $kind:ident),* $(,)?) => {$(
        impl FieldValue for $type {
            const SIZE: usize = size_of::<$type>();

            fn fits(kind: TypeKind) -> bool {
                kind == TypeKind::$kind
            }

            fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
                Ok(Primitive::from_le_slice(bytes))
            }

            fn encode(&self) -> Vec<u8> {
                Primitive::to_le_vec(*self)
            }
        }
    )*};
}

primitive_field_values!(
    u8 => Byte,
    i16 => Short,
    u16 => UShort,
    i32 => Int,
    u32 => UInt,
    i64 => Long,
    u64 => ULong,
    f32 => Float,
    f64 => Double,
);

impl FieldValue for bool {
    const SIZE: usize = 1;

    fn fits(kind: TypeKind) -> bool {
        kind == TypeKind::Bool
    }

    fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Ok(bytes[0] != 0)
    }

    fn encode(&self) -> Vec<u8> {
        vec![u8::from(*self)]
    }
}

/// A C# enum, read as a Rust enum with the same values.
///
/// The game's enums are `int`s, so give the Rust enum `#[repr(i32)]` and derive `num_enum`'s
/// `TryFromPrimitive` and `IntoPrimitive`. Reading a value the Rust enum doesn't have fails.
pub trait Enum: Copy + TryFrom<i32> + Into<i32> {}

/// An [`Enum`] with a value that means "nothing", like `HelmType.None` for a zombie without
/// headwear. A field of it can be read as an `Option`, which is `None` for that value.
pub trait EnumWithNone: Enum {
    /// The value that means "nothing".
    const NONE: i32;
}

impl<E: Enum> FieldValue for E {
    const SIZE: usize = size_of::<i32>();

    fn fits(kind: TypeKind) -> bool {
        kind == TypeKind::ValueType
    }

    fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        let value = i32::from_le_slice(bytes);
        E::try_from(value).map_err(|_| DecodeError::UnknownValue {
            value,
            type_name: short_type_name::<E>(),
        })
    }

    fn encode(&self) -> Vec<u8> {
        (*self).into().to_le_vec()
    }
}

impl<E: EnumWithNone> FieldValue for Option<E> {
    const SIZE: usize = E::SIZE;

    fn fits(kind: TypeKind) -> bool {
        E::fits(kind)
    }

    fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        if i32::from_le_slice(bytes) == E::NONE {
            Ok(None)
        } else {
            E::decode(bytes).map(Some)
        }
    }

    fn encode(&self) -> Vec<u8> {
        match self {
            Some(value) => value.encode(),
            None => E::NONE.to_le_vec(),
        }
    }
}

/// A reference that may be null.
impl FieldValue for Option<Object> {
    const SIZE: usize = size_of::<usize>();

    fn fits(kind: TypeKind) -> bool {
        kind.is_reference()
    }

    fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Ok(Object::at(usize::from_le_slice(bytes)))
    }

    fn encode(&self) -> Vec<u8> {
        self.map_or(0, Object::address).to_le_vec()
    }
}

/// A reference that's never null. Reading a null one fails.
impl FieldValue for Object {
    const SIZE: usize = size_of::<usize>();

    fn fits(kind: TypeKind) -> bool {
        kind.is_reference()
    }

    fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Option::<Object>::decode(bytes)?.ok_or(DecodeError::Null)
    }

    fn encode(&self) -> Vec<u8> {
        Some(*self).encode()
    }
}

impl<T> FieldValue for Option<Ref<T>> {
    const SIZE: usize = size_of::<usize>();

    fn fits(kind: TypeKind) -> bool {
        kind.is_reference()
    }

    fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Ok(Option::<Object>::decode(bytes)?.map(Ref::from))
    }

    fn encode(&self) -> Vec<u8> {
        self.map(Ref::object).encode()
    }
}

impl<T> FieldValue for Ref<T> {
    const SIZE: usize = size_of::<usize>();

    fn fits(kind: TypeKind) -> bool {
        kind.is_reference()
    }

    fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Object::decode(bytes).map(Ref::from)
    }

    fn encode(&self) -> Vec<u8> {
        self.object().encode()
    }
}

/// A type's name without its module path, like `PlantType`, for error messages.
fn short_type_name<T>() -> &'static str {
    let name = std::any::type_name::<T>();
    name.rsplit("::").next().unwrap_or(name)
}

#[cfg(test)]
mod tests {
    use num_enum::{IntoPrimitive, TryFromPrimitive};

    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive, IntoPrimitive)]
    #[repr(i32)]
    enum Headwear {
        Cone = 1,
        Bucket = 2,
    }

    impl Enum for Headwear {}

    impl EnumWithNone for Headwear {
        const NONE: i32 = 0;
    }

    #[test]
    fn enums_round_trip_and_reject_unknown_values() {
        let bucket = Headwear::Bucket.encode();
        assert_eq!(bucket, [2, 0, 0, 0]);
        assert_eq!(Headwear::decode(&bucket).unwrap(), Headwear::Bucket);
        assert!(matches!(
            Headwear::decode(&[7, 0, 0, 0]),
            Err(DecodeError::UnknownValue {
                value: 7,
                type_name: "Headwear"
            })
        ));
    }

    #[test]
    fn the_none_value_reads_as_none() {
        assert_eq!(Option::<Headwear>::decode(&[0; 4]).unwrap(), None);
        assert_eq!(
            Option::<Headwear>::decode(&[1, 0, 0, 0]).unwrap(),
            Some(Headwear::Cone)
        );
        assert_eq!(Option::<Headwear>::None.encode(), [0; 4]);
    }

    #[test]
    fn null_references() {
        assert_eq!(Option::<Object>::decode(&[0; 8]).unwrap(), None);
        assert!(matches!(Object::decode(&[0; 8]), Err(DecodeError::Null)));
        let object = Object::decode(&0x1234_5678_usize.to_le_bytes()).unwrap();
        assert_eq!(object.address(), 0x1234_5678);
    }
}
