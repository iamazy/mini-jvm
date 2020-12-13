/// All constant_pool table entries have the following general format:
/// ```txt
/// cp_info {
///     u1 tag;
///     u1 info[];
/// }
/// ```
///
/// Constant pool tags:
///
/// ```txt
/// CONSTANT_Class -> 7
/// CONSTANT_Fieldref -> 9
/// CONSTANT_Methodref -> 10
/// CONSTANT_InterfaceMethodref -> 11
/// CONSTANT_String -> 8
/// CONSTANT_Integer -> 3
/// CONSTANT_Float -> 4
/// CONSTANT_Long -> 5
/// CONSTANT_Double -> 6
/// CONSTANT_NameAndType -> 12
/// CONSTANT_Utf8 -> 1
/// CONSTANT_MethodHandle -> 15
/// CONSTANT_MethodType -> 16
/// CONSTANT_InvokeDynamic -> 18
/// ```
#[derive(Debug)]
pub enum Constant {
    Class {
        name_index: u16,
    },
    FieldRef {
        // The class_index item of a CONSTANT_Fieldref_info structure may be either a class type or an interface type.
        class_index: u16,
        name_and_type_index: u16
    },
    MethodRef {
        // The class_index item of a CONSTANT_Methodref_info structure must be a class type, not an interface type.
        class_index: u16,
        // If the name of the method of a CONSTANT_Methodref_info structure begins with a '<' ('\u003c'),
        // then the name must be the special name <init>, representing an instance initialization method.
        // The return type of such a method must be void.
        name_and_type_index: u16
    },
    InterfaceMethodRef {
        // The class_index item of a CONSTANT_InterfaceMethodref_info structure must be an interface type.
        class_index: u16,
        name_and_type_index: u16
    },
    String {
        string_index: u16
    },
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    NameAndType {
        name_index: u16,
        descriptor_index: u16
    },
    Utf8(String),
    MethodHandle {
        reference_kind: u8,
        reference_index: u8
    },
    MethodType {
        descriptor_index: u16
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16
    }
}