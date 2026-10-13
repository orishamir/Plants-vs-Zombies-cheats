use std::collections::HashMap;
use std::sync::{Arc, Mutex, PoisonError};

use super::value::DecodeError;
use super::{FieldValue, ObjectError, TypeKind, runtime};
use crate::game::Primitive;
use crate::{Game, Location};

/// One of the game's C# classes or structs, like `Reloaded.Gameplay.Plant`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Class(usize);

/// One of the `<class>_TypeInfo` globals in `GameAssembly.dll`, where IL2CPP keeps the
/// address of a class once the game has used it. Il2CppDumper's `script.json` lists them with
/// their addresses, which change with every game update.
#[derive(Debug, Clone, Copy)]
pub struct TypeInfo {
    /// Its name in `script.json`, like `Reloaded.Gameplay.Board_TypeInfo`.
    pub name: &'static str,
    pub location: Location,
}

impl TypeInfo {
    /// The class this global points to.
    pub fn class(self, game: &Game) -> Result<Class, ObjectError> {
        match game.read_pointer(self.location.resolve(game)?)? {
            0 => Err(ObjectError::ClassNotLoaded {
                type_info: self.name,
            }),
            address => Ok(Class(address)),
        }
    }
}

/// A field of a class.
#[derive(Debug, Clone, Copy)]
pub(super) struct Field {
    /// Where the field is: in an object, counting from the start of the object, or for a
    /// static field, in the class's static fields.
    pub offset: usize,
    pub kind: TypeKind,
    pub is_static: bool,
}

/// What's been read about a class.
#[derive(Debug)]
pub(super) struct ClassInfo {
    /// Like `Reloaded.Gameplay.Plant`. A generic class's name ends with how many type
    /// parameters it has, like ``System.Collections.Generic.List`1``.
    full_name: String,
    parent: Option<Class>,
    /// The fields the class declares, without the ones it inherits.
    fields: HashMap<String, Field>,
    /// What kind of type the class is, like a value type for an `int` or a struct.
    pub kind: TypeKind,
    pub instance_size: usize,
    /// For an array class, the class of its elements and how many bytes each one takes.
    pub element: Option<(Class, usize)>,
    /// For an array class, how many dimensions its arrays have.
    pub rank: u8,
}

/// What's been read about the game's classes. IL2CPP never moves or frees a class while the
/// game runs, so each class only needs to be read once.
#[derive(Debug, Default)]
pub(crate) struct ClassCache(Mutex<HashMap<Class, Arc<ClassInfo>>>);

/// An `Il2CppType`'s bitfield (see [`runtime::type_::BITS`]).
#[derive(Debug, Clone, Copy)]
struct TypeBits(u32);

impl TypeBits {
    fn attributes(self) -> u16 {
        self.0 as u16
    }

    fn kind(self) -> TypeKind {
        TypeKind::from((self.0 >> 16) as u8)
    }
}

impl Class {
    pub(super) fn at(address: usize) -> Self {
        Self(address)
    }

    /// Like `Reloaded.Gameplay.Plant`.
    pub fn full_name(self, game: &Game) -> Result<String, ObjectError> {
        Ok(self.info(game)?.full_name.clone())
    }

    /// Reads one of the class's static fields.
    pub fn get_static<T: FieldValue>(
        self,
        game: &Game,
        name: &'static str,
    ) -> Result<T, ObjectError> {
        let field = self.field::<T>(game, name, true)?;
        let statics = game.read_pointer(self.0 + runtime::class::STATIC_FIELDS)?;
        let bytes = game.read_memory(statics + field.offset, T::SIZE)?;
        T::decode(&bytes).map_err(|error| self.invalid_value(game, name, error))
    }

    /// Finds the instance field `name`, in this class or one it inherits from, and checks that
    /// it can be read as `T`.
    pub(super) fn instance_field<T: FieldValue>(
        self,
        game: &Game,
        name: &'static str,
    ) -> Result<Field, ObjectError> {
        self.field::<T>(game, name, false)
    }

    fn field<T: FieldValue>(
        self,
        game: &Game,
        name: &'static str,
        is_static: bool,
    ) -> Result<Field, ObjectError> {
        let mut found = None;
        for class in self.ancestors(game) {
            if let Some(&field) = class?.fields.get(name) {
                found = Some(field);
                break;
            }
        }
        let field = found
            .filter(|field| field.is_static == is_static)
            .ok_or_else(|| ObjectError::MissingField {
                class: self.describe(game),
                field: name,
            })?;

        if T::fits(field.kind) {
            Ok(field)
        } else {
            Err(ObjectError::WrongFieldType {
                class: self.describe(game),
                field: name,
                found: field.kind,
                expected: std::any::type_name::<T>(),
            })
        }
    }

    /// Whether this class is the one named `full_name`, or inherits from it. A generic class
    /// matches without its type parameters: ``List`1`` is `System.Collections.Generic.List`.
    pub(super) fn is_a(self, game: &Game, full_name: &str) -> Result<bool, ObjectError> {
        for class in self.ancestors(game) {
            if class?.full_name.split('`').next() == Some(full_name) {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// This class, then the class it inherits from, and so on up to `System.Object`.
    fn ancestors(self, game: &Game) -> impl Iterator<Item = Result<Arc<ClassInfo>, ObjectError>> {
        let mut next = Some(self);
        std::iter::from_fn(move || {
            let info = next?.info(game);
            next = info.as_ref().ok().and_then(|info| info.parent);
            Some(info)
        })
    }

    /// The error for a field whose value isn't one the field's Rust type can hold.
    pub(super) fn invalid_value(self, game: &Game, field: &str, error: DecodeError) -> ObjectError {
        ObjectError::InvalidValue {
            place: format!("{}.{field}", self.describe(game)),
            error,
        }
    }

    /// The class's name for error messages, even if it can't be read.
    pub(super) fn describe(self, game: &Game) -> String {
        self.full_name(game)
            .unwrap_or_else(|_| format!("the class at {:#X}", self.0))
    }

    pub(super) fn info(self, game: &Game) -> Result<Arc<ClassInfo>, ObjectError> {
        let cache = &game.classes().0;
        let cached = cache
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .get(&self)
            .cloned();
        if let Some(info) = cached {
            return Ok(info);
        }

        let info = Arc::new(ClassInfo::read(game, self)?);
        cache
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(self, Arc::clone(&info));
        Ok(info)
    }
}

impl ClassInfo {
    fn read(game: &Game, class: Class) -> Result<Self, ObjectError> {
        use runtime::class::*;

        let header = game.read_memory(class.0, SIZE)?;
        let pointer_at = |offset| usize::from_le_slice_at(&header, offset);
        let u32_at = |offset| u32::from_le_slice_at(&header, offset);

        let namespace = read_c_string(game, pointer_at(NAMESPACE))?;
        let name = read_c_string(game, pointer_at(NAME))?;
        let full_name = if namespace.is_empty() {
            name
        } else {
            format!("{namespace}.{name}")
        };

        let kind = TypeBits(u32_at(TYPE + runtime::type_::BITS)).kind();
        let element = matches!(kind, TypeKind::Array | TypeKind::MultiArray).then(|| {
            (
                Class(pointer_at(ELEMENT_CLASS)),
                u32_at(ELEMENT_SIZE) as usize,
            )
        });

        let field_count = u16::from_le_slice_at(&header, FIELD_COUNT);
        let fields = match (field_count, pointer_at(FIELDS)) {
            (0, _) => HashMap::new(),
            // IL2CPP sets up a class's fields the first time the game uses the class.
            (_, 0) => return Err(ObjectError::FieldsNotLoaded { class: full_name }),
            (count, address) => read_fields(game, address, count.into())?,
        };

        Ok(Self {
            full_name,
            parent: Some(pointer_at(PARENT))
                .filter(|&address| address != 0)
                .map(Class),
            fields,
            kind,
            instance_size: u32_at(INSTANCE_SIZE) as usize,
            element,
            rank: header[RANK],
        })
    }
}

/// Reads `count` `FieldInfo`s at `address`, by their names.
fn read_fields(
    game: &Game,
    address: usize,
    count: usize,
) -> Result<HashMap<String, Field>, ObjectError> {
    use runtime::{field, type_};

    let table = game.read_memory(address, count * field::SIZE)?;
    let mut fields = HashMap::with_capacity(count);
    for info in table.as_chunks::<{ field::SIZE }>().0 {
        let type_address = usize::from_le_slice_at(info, field::TYPE);
        let type_bits = TypeBits(game.read(type_address + type_::BITS)?);
        let offset = i32::from_le_slice_at(info, field::OFFSET);

        // A `const` has no storage, and a `[ThreadStatic]` field's offset is -1, since each
        // thread has its own.
        let Ok(offset) = usize::try_from(offset) else {
            continue;
        };
        if type_bits.attributes() & type_::LITERAL != 0 {
            continue;
        }

        let name_address = usize::from_le_slice_at(info, field::NAME);
        fields.insert(
            read_c_string(game, name_address)?,
            Field {
                offset,
                kind: type_bits.kind(),
                is_static: type_bits.attributes() & type_::STATIC != 0,
            },
        );
    }
    Ok(fields)
}

/// Reads a NUL-terminated UTF-8 string, like a class or field name.
fn read_c_string(game: &Game, address: usize) -> Result<String, ObjectError> {
    const CHUNK: usize = 64;
    const PAGE: usize = 0x1000;
    const LONGEST: usize = 1024;

    let mut bytes = Vec::new();
    loop {
        let at = address + bytes.len();
        // Never read past the end of a page, which could be the end of readable memory.
        let chunk = game.read_memory(at, CHUNK.min(PAGE - at % PAGE))?;
        match chunk.iter().position(|&byte| byte == 0) {
            Some(end) => {
                bytes.extend_from_slice(&chunk[..end]);
                break;
            }
            None if bytes.len() < LONGEST => bytes.extend(chunk),
            None => return Err(ObjectError::InvalidName { address }),
        }
    }
    String::from_utf8(bytes).map_err(|_| ObjectError::InvalidName { address })
}
