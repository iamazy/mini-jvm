use crate::{FromToBytes, read_string, write_string};
use bytes::{BytesMut, BufMut, Buf};
use crate::error::Error;
use crate::ops;
use std::fmt::{self, Formatter};

///```jvm
/// cp_info {
///     u1 tag;
///     u1 info[];
/// }
///```
pub fn get_utf8(constant_pool: &Vec<Constant>, index: usize) -> Option<&String> {
    let constant = &constant_pool[index - 1];
    return if let Constant::Utf8(string) = constant {
        Some(&string)
    } else {
        None
    };
}

pub fn get_class_name(constant_pool: &Vec<Constant>, index: usize) -> Option<&String> {
    let constant = &constant_pool[index - 1];
    return if let Constant::Class { name_index } = constant {
        get_utf8(constant_pool, *name_index as usize)
    } else {
        None
    };
}


#[derive(Debug, Clone)]
pub enum Constant {
    Class {
        name_index: u16,
    },
    FieldRef {
        // The class_index item of a CONSTANT_Fieldref_info structure may be either a class type or an interface type.
        class_index: u16,
        name_and_type_index: u16,
    },
    MethodRef {
        // The class_index item of a CONSTANT_Methodref_info structure must be a class type, not an interface type.
        class_index: u16,
        // If the name of the method of a CONSTANT_Methodref_info structure begins with a '<' ('\u003c'),
        // then the name must be the special name <init>, representing an instance initialization method.
        // The return type of such a method must be void.
        name_and_type_index: u16,
    },
    InterfaceMethodRef {
        // The class_index item of a CONSTANT_InterfaceMethodref_info structure must be an interface type.
        class_index: u16,
        name_and_type_index: u16,
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
        descriptor_index: u16,
    },
    Utf8(String),
    MethodHandle {
        reference_kind: u8,
        reference_index: u16,
    },
    MethodType {
        descriptor_index: u16
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
}

impl FromToBytes<Constant> for Constant {
    fn to_buf(&self, buf: &mut impl BufMut) -> Result<usize, Error> {
        let mut len: usize = 1;
        match self {
            Constant::Class { name_index } => {
                buf.put_u8(ops::CONSTANT_CLASS_TAG);
                buf.put_u16(*name_index);
                len += 2;
            }
            Constant::FieldRef { class_index, name_and_type_index } => {
                buf.put_u8(ops::CONSTANT_FIELD_REF_TAG);
                buf.put_u16(*class_index);
                buf.put_u16(*name_and_type_index);
                len += 4;
            }
            Constant::MethodRef { class_index, name_and_type_index } => {
                buf.put_u8(ops::CONSTANT_METHOD_REF_TAG);
                buf.put_u16(*class_index);
                buf.put_u16(*name_and_type_index);
                len += 4;
            }
            Constant::InterfaceMethodRef { class_index, name_and_type_index } => {
                buf.put_u8(ops::CONSTANT_INTERFACE_METHOD_REF_TAG);
                buf.put_u16(*class_index);
                buf.put_u16(*name_and_type_index);
                len += 4;
            }
            Constant::String { string_index } => {
                buf.put_u8(ops::CONSTANT_STRING_TAG);
                buf.put_u16(*string_index);
                len += 2;
            }
            Constant::Integer(int) => {
                buf.put_u8(ops::CONSTANT_INTEGER_TAG);
                buf.put_i32(*int);
                len += 4;
            }
            Constant::Float(float) => {
                buf.put_u8(ops::CONSTANT_FLOAT_TAG);
                buf.put_f32(*float);
                len += 4;
            }
            Constant::Long(long) => {
                buf.put_u8(ops::CONSTANT_LONG_TAG);
                buf.put_i64(*long);
                len += 8;
            }
            Constant::Double(double) => {
                buf.put_u8(ops::CONSTANT_DOUBLE_TAG);
                buf.put_f64(*double);
                len += 8;
            }
            Constant::NameAndType { name_index, descriptor_index } => {
                buf.put_u8(ops::CONSTANT_NAME_AND_TYPE_TAG);
                buf.put_u16(*name_index);
                buf.put_u16(*descriptor_index);
                len += 4;
            }
            Constant::Utf8(string) => {
                buf.put_u8(ops::CONSTANT_UTF8_TAG);
                len += write_string((*string).clone(), buf);
            }
            Constant::MethodHandle { reference_kind, reference_index } => {
                buf.put_u8(ops::CONSTANT_METHOD_HANDLE_TAG);
                buf.put_u8(*reference_kind);
                buf.put_u16(*reference_index);
                len += 3;
            }
            Constant::MethodType { descriptor_index } => {
                buf.put_u8(ops::CONSTANT_METHOD_TYPE_TAG);
                buf.put_u16(*descriptor_index);
                len += 2;
            }
            Constant::InvokeDynamic { bootstrap_method_attr_index, name_and_type_index } => {
                buf.put_u8(ops::CONSTANT_INVOKE_DYNAMIC_TAG);
                buf.put_u16(*bootstrap_method_attr_index);
                buf.put_u16(*name_and_type_index);
                len += 4;
            }
        }
        Ok(len)
    }

    fn from_buf(buf: &mut BytesMut) -> Result<Constant, Error> {
        let tag = buf.get_u8();
        match tag {
            ops::CONSTANT_CLASS_TAG => {
                let name_index = buf.get_u16();
                Ok(Constant::Class { name_index })
            }
            ops::CONSTANT_FIELD_REF_TAG => {
                let class_index = buf.get_u16();
                let name_and_type_index = buf.get_u16();
                Ok(Constant::FieldRef { class_index, name_and_type_index })
            }
            ops::CONSTANT_METHOD_REF_TAG => {
                let class_index = buf.get_u16();
                let name_and_type_index = buf.get_u16();
                Ok(Constant::MethodRef { class_index, name_and_type_index })
            }
            ops::CONSTANT_INTERFACE_METHOD_REF_TAG => {
                let class_index = buf.get_u16();
                let name_and_type_index = buf.get_u16();
                Ok(Constant::InterfaceMethodRef { class_index, name_and_type_index })
            }
            ops::CONSTANT_STRING_TAG => {
                let string_index = buf.get_u16();
                Ok(Constant::String { string_index })
            }
            ops::CONSTANT_INTEGER_TAG => {
                let value = buf.get_i32();
                Ok(Constant::Integer(value))
            }
            ops::CONSTANT_FLOAT_TAG => {
                let value = buf.get_f32();
                Ok(Constant::Float(value))
            }
            ops::CONSTANT_LONG_TAG => {
                let value = buf.get_i64();
                Ok(Constant::Long(value))
            }
            ops::CONSTANT_DOUBLE_TAG => {
                let value = buf.get_f64();
                Ok(Constant::Double(value))
            }
            ops::CONSTANT_NAME_AND_TYPE_TAG => {
                let name_index = buf.get_u16();
                let descriptor_index = buf.get_u16();
                Ok(Constant::NameAndType { name_index, descriptor_index })
            }
            ops::CONSTANT_UTF8_TAG => {
                Ok(Constant::Utf8(read_string(buf)?))
            }
            ops::CONSTANT_METHOD_HANDLE_TAG => {
                let reference_kind = buf.get_u8();
                let reference_index = buf.get_u16();
                Ok(Constant::MethodHandle { reference_kind, reference_index })
            }
            ops::CONSTANT_METHOD_TYPE_TAG => {
                let descriptor_index = buf.get_u16();
                Ok(Constant::MethodType { descriptor_index })
            }
            ops::CONSTANT_INVOKE_DYNAMIC_TAG => {
                let bootstrap_method_attr_index = buf.get_u16();
                let name_and_type_index = buf.get_u16();
                Ok(Constant::InvokeDynamic { bootstrap_method_attr_index, name_and_type_index })
            }
            _ => Err(Error::InvalidConstantTag(tag))
        }
    }

    fn length(&self) -> usize {
        unimplemented!()
    }
}

impl fmt::Display for Constant {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Constant::Class { .. } => "Constant::Class".fmt(fmt),
            Constant::FieldRef { .. } => "Constant::FieldRef".fmt(fmt),
            Constant::MethodRef { .. } => "Constant::MethodRef".fmt(fmt),
            Constant::InterfaceMethodRef { .. } => "Constant::InterfaceMethodRef".fmt(fmt),
            Constant::String { .. } => "Constant::String".fmt(fmt),
            Constant::Integer(..) => "Constant::Integer".fmt(fmt),
            Constant::Float(..) => "Constant::Float".fmt(fmt),
            Constant::Long(..) => "Constant::Long".fmt(fmt),
            Constant::Double(..) => "Constant::Double".fmt(fmt),
            Constant::NameAndType { .. } => "Constant::NameAndType".fmt(fmt),
            Constant::Utf8(..) => "Constant::Utf8".fmt(fmt),
            Constant::MethodHandle { .. } => "Constant::MethodHandle".fmt(fmt),
            Constant::MethodType { .. } => "Constant::MethodType".fmt(fmt),
            Constant::InvokeDynamic { .. } => "Constant::InvokeDynamic".fmt(fmt)
        }
    }
}