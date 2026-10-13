//! Where IL2CPP keeps things in its own structs, for the game's version of Unity (6000.0).
//!
//! Each offset comes from the code of one of `GameAssembly.dll`'s exported `il2cpp_*`
//! functions, named next to it. Most are one instruction, like `il2cpp_class_get_parent`'s
//! `mov rax,[rcx+58h]`, which `re-tools bytes` and a disassembler show. Il2CppDumper's
//! `il2cpp.h` has the same structs (`Il2CppClass_1`, `Il2CppClass_2`), but isn't exact for
//! this Unity version.
//!
//! These only change when the game moves to a new version of Unity.

/// `Il2CppObject`, the start of every object.
pub mod object {
    /// The object's `Il2CppClass*`. (`il2cpp_object_get_class`)
    pub const CLASS: usize = 0x0;
    /// How many bytes the class pointer and the monitor take. Field offsets count them, so the
    /// fields of a struct stored inside an array start this much earlier than their offsets.
    pub const HEADER_SIZE: usize = 0x10;
}

/// `Il2CppClass`.
pub mod class {
    /// `const char*`. (`il2cpp_class_get_name`)
    pub const NAME: usize = 0x10;
    /// `const char*`. (`il2cpp_class_get_namespace`)
    pub const NAMESPACE: usize = 0x18;
    /// The class's own `Il2CppType`. (`il2cpp_class_get_type`)
    pub const TYPE: usize = 0x20;
    /// For an array class, the class of its elements. (`il2cpp_class_get_element_class`)
    pub const ELEMENT_CLASS: usize = 0x40;
    /// `Il2CppClass*`, null for `System.Object`. (`il2cpp_class_get_parent`)
    pub const PARENT: usize = 0x58;
    /// `FieldInfo*`, an array of [`field_count`](FIELD_COUNT). (`il2cpp_class_get_fields`)
    pub const FIELDS: usize = 0x80;
    /// Where the class's static fields are. (`il2cpp_class_get_static_field_data`)
    pub const STATIC_FIELDS: usize = 0xB8;
    /// `u32`, the size of an object of the class. (`il2cpp_class_instance_size`)
    pub const INSTANCE_SIZE: usize = 0xF8;
    /// `u32`, for an array class, the size of one element. (`il2cpp_array_element_size`)
    pub const ELEMENT_SIZE: usize = 0x104;
    /// `u16`, how many fields the class declares, without inherited ones.
    /// (`il2cpp_class_num_fields`)
    pub const FIELD_COUNT: usize = 0x124;
    /// How many bytes to read to get everything above.
    pub const SIZE: usize = 0x128;
}

/// `FieldInfo`.
pub mod field {
    /// `const char*`. (`il2cpp_field_get_name`)
    pub const NAME: usize = 0x0;
    /// `Il2CppType*`. (`il2cpp_field_get_type`)
    pub const TYPE: usize = 0x8;
    /// `i32`: where the field is in an object, or in the class's static fields.
    /// (`il2cpp_field_get_offset`)
    pub const OFFSET: usize = 0x18;
    /// The size of a `FieldInfo`, as `il2cpp_class_get_fields` steps through them.
    pub const SIZE: usize = 0x20;
}

/// `Il2CppType`.
pub mod type_ {
    /// `u32`, a bitfield. Bits 0 to 15 are a field's attributes (`il2cpp_type_get_attrs`),
    /// and bits 16 to 23 are the kind of type, an `Il2CppTypeEnum` (`il2cpp_type_get_type`).
    pub const BITS: usize = 0x8;
    /// `FIELD_ATTRIBUTE_STATIC`.
    pub const STATIC: u16 = 0x10;
    /// `FIELD_ATTRIBUTE_LITERAL`: a `const`, which has no storage.
    pub const LITERAL: u16 = 0x40;
}

/// `Il2CppArray`.
pub mod array {
    /// `usize`, how many elements the array has. (`il2cpp_array_length`)
    pub const LENGTH: usize = 0x18;
    /// Where the elements start. (`il2cpp_array_object_header_size`)
    pub const ELEMENTS: usize = 0x20;
}

/// `Il2CppString`, C#'s `string`.
pub mod string {
    /// `i32`, how many UTF-16 code units the string has. (`il2cpp_string_length`)
    pub const LENGTH: usize = 0x10;
    /// Where the UTF-16 code units start. (`il2cpp_string_chars`)
    pub const CHARS: usize = 0x14;
}
