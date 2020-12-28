use crate::{read_string, write_string, TryInto};
use bytes::{BytesMut, BufMut, Buf};
use crate::error::Error;
use std::fmt::{self, Formatter};
use std::convert::TryFrom;

/// Tag values for the constant pool entries
#[derive(Debug, Clone)]
pub enum Tag {
    Class,
    FieldRef,
    MethodRef,
    InterfaceMethodRef,
    String,
    Integer,
    Float,
    Long,
    Double,
    NameAndType,
    Utf8,
    MethodHandle,
    MethodType,
    Dynamic,
    InvokeDynamic,
    Module,
    Package
}

impl Into<u8> for Tag {
    fn into(self) -> u8 {
        match self {
            Tag::Class => 7u8,
            Tag::FieldRef => 9u8,
            Tag::MethodRef => 10u8,
            Tag::InterfaceMethodRef => 11u8,
            Tag::String => 8u8,
            Tag::Integer => 3u8,
            Tag::Float => 4u8,
            Tag::Long => 5u8,
            Tag::Double => 6u8,
            Tag::NameAndType => 12u8,
            Tag::Utf8 => 1u8,
            Tag::MethodHandle => 15u8,
            Tag::MethodType => 16u8,
            Tag::Dynamic => 17u8,
            Tag::InvokeDynamic => 18u8,
            Tag::Module => 19u8,
            Tag::Package => 20u8,
        }
    }
}

impl From<u8> for Tag {
    fn from(tag: u8) -> Self {
        match tag {
            7 => Tag::Class,
            9 => Tag::FieldRef,
            10 => Tag::MethodRef,
            11 => Tag::InterfaceMethodRef,
            8 => Tag::String,
            3 => Tag::Integer,
            4 => Tag::Float,
            5 => Tag::Long,
            6 => Tag::Double,
            12 => Tag::NameAndType,
            1 => Tag::Utf8,
            15 => Tag::MethodHandle,
            16 => Tag::MethodType,
            17 => Tag::Dynamic,
            18 => Tag::InvokeDynamic,
            19 => Tag::Module,
            20 => Tag::Package,
            _ => unreachable!()
        }
    }
}

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

impl TryFrom<&mut BytesMut> for Constant {
    type Error = Error;

    fn try_from(buf: &mut BytesMut) -> Result<Self, Self::Error> {
        let tag = buf.get_u8();
        let tag = Tag::from(tag);
        match tag {
            Tag::Class => {
                let name_index = buf.get_u16();
                Ok(Constant::Class { name_index })
            }
            Tag::FieldRef => {
                let class_index = buf.get_u16();
                let name_and_type_index = buf.get_u16();
                Ok(Constant::FieldRef { class_index, name_and_type_index })
            }
            Tag::MethodRef => {
                let class_index = buf.get_u16();
                let name_and_type_index = buf.get_u16();
                Ok(Constant::MethodRef { class_index, name_and_type_index })
            }
            Tag::InterfaceMethodRef => {
                let class_index = buf.get_u16();
                let name_and_type_index = buf.get_u16();
                Ok(Constant::InterfaceMethodRef { class_index, name_and_type_index })
            }
            Tag::String => {
                let string_index = buf.get_u16();
                Ok(Constant::String { string_index })
            }
            Tag::Integer => {
                let value = buf.get_i32();
                Ok(Constant::Integer(value))
            }
            Tag::Float => {
                let value = buf.get_f32();
                Ok(Constant::Float(value))
            }
            Tag::Long => {
                let value = buf.get_i64();
                Ok(Constant::Long(value))
            }
            Tag::Double => {
                let value = buf.get_f64();
                Ok(Constant::Double(value))
            }
            Tag::NameAndType => {
                let name_index = buf.get_u16();
                let descriptor_index = buf.get_u16();
                Ok(Constant::NameAndType { name_index, descriptor_index })
            }
            Tag::Utf8 => {
                Ok(Constant::Utf8(read_string(buf)?))
            }
            Tag::MethodHandle => {
                let reference_kind = buf.get_u8();
                let reference_index = buf.get_u16();
                Ok(Constant::MethodHandle { reference_kind, reference_index })
            }
            Tag::MethodType => {
                let descriptor_index = buf.get_u16();
                Ok(Constant::MethodType { descriptor_index })
            }
            Tag::InvokeDynamic => {
                let bootstrap_method_attr_index = buf.get_u16();
                let name_and_type_index = buf.get_u16();
                Ok(Constant::InvokeDynamic { bootstrap_method_attr_index, name_and_type_index })
            }
            _ => Err(Error::InvalidConstantTag(tag.into()))
        }
    }
}

impl<T> TryInto<&mut T, usize> for Constant where
    T: BufMut {
    type Error = Error;

    fn try_into(&self, buf: &mut T) -> Result<usize, Self::Error> {
        let mut len: usize = 1;
        match self {
            Constant::Class { name_index } => {
                buf.put_u8(Tag::Class.into());
                buf.put_u16(*name_index);
                len += 2;
            }
            Constant::FieldRef { class_index, name_and_type_index } => {
                buf.put_u8(Tag::FieldRef.into());
                buf.put_u16(*class_index);
                buf.put_u16(*name_and_type_index);
                len += 4;
            }
            Constant::MethodRef { class_index, name_and_type_index } => {
                buf.put_u8(Tag::MethodRef.into());
                buf.put_u16(*class_index);
                buf.put_u16(*name_and_type_index);
                len += 4;
            }
            Constant::InterfaceMethodRef { class_index, name_and_type_index } => {
                buf.put_u8(Tag::InterfaceMethodRef.into());
                buf.put_u16(*class_index);
                buf.put_u16(*name_and_type_index);
                len += 4;
            }
            Constant::String { string_index } => {
                buf.put_u8(Tag::String.into());
                buf.put_u16(*string_index);
                len += 2;
            }
            Constant::Integer(int) => {
                buf.put_u8(Tag::Integer.into());
                buf.put_i32(*int);
                len += 4;
            }
            Constant::Float(float) => {
                buf.put_u8(Tag::Float.into());
                buf.put_f32(*float);
                len += 4;
            }
            Constant::Long(long) => {
                buf.put_u8(Tag::Long.into());
                buf.put_i64(*long);
                len += 8;
            }
            Constant::Double(double) => {
                buf.put_u8(Tag::Double.into());
                buf.put_f64(*double);
                len += 8;
            }
            Constant::NameAndType { name_index, descriptor_index } => {
                buf.put_u8(Tag::NameAndType.into());
                buf.put_u16(*name_index);
                buf.put_u16(*descriptor_index);
                len += 4;
            }
            Constant::Utf8(string) => {
                buf.put_u8(Tag::Utf8.into());
                len += write_string(string.clone(), buf);
            }
            Constant::MethodHandle { reference_kind, reference_index } => {
                buf.put_u8(Tag::MethodHandle.into());
                buf.put_u8(*reference_kind);
                buf.put_u16(*reference_index);
                len += 3;
            }
            Constant::MethodType { descriptor_index } => {
                buf.put_u8(Tag::MethodType.into());
                buf.put_u16(*descriptor_index);
                len += 2;
            }
            Constant::InvokeDynamic { bootstrap_method_attr_index, name_and_type_index } => {
                buf.put_u8(Tag::InvokeDynamic.into());
                buf.put_u16(*bootstrap_method_attr_index);
                buf.put_u16(*name_and_type_index);
                len += 4;
            }
        }
        Ok(len)
    }
}

impl Constant {
    pub fn to_buf(&self, buf: &mut impl BufMut) -> Result<usize, Error> {
        let mut len: usize = 1;
        match self {
            Constant::Class { name_index } => {
                buf.put_u8(Tag::Class.into());
                buf.put_u16(*name_index);
                len += 2;
            }
            Constant::FieldRef { class_index, name_and_type_index } => {
                buf.put_u8(Tag::FieldRef.into());
                buf.put_u16(*class_index);
                buf.put_u16(*name_and_type_index);
                len += 4;
            }
            Constant::MethodRef { class_index, name_and_type_index } => {
                buf.put_u8(Tag::MethodRef.into());
                buf.put_u16(*class_index);
                buf.put_u16(*name_and_type_index);
                len += 4;
            }
            Constant::InterfaceMethodRef { class_index, name_and_type_index } => {
                buf.put_u8(Tag::InterfaceMethodRef.into());
                buf.put_u16(*class_index);
                buf.put_u16(*name_and_type_index);
                len += 4;
            }
            Constant::String { string_index } => {
                buf.put_u8(Tag::String.into());
                buf.put_u16(*string_index);
                len += 2;
            }
            Constant::Integer(int) => {
                buf.put_u8(Tag::Integer.into());
                buf.put_i32(*int);
                len += 4;
            }
            Constant::Float(float) => {
                buf.put_u8(Tag::Float.into());
                buf.put_f32(*float);
                len += 4;
            }
            Constant::Long(long) => {
                buf.put_u8(Tag::Long.into());
                buf.put_i64(*long);
                len += 8;
            }
            Constant::Double(double) => {
                buf.put_u8(Tag::Double.into());
                buf.put_f64(*double);
                len += 8;
            }
            Constant::NameAndType { name_index, descriptor_index } => {
                buf.put_u8(Tag::NameAndType.into());
                buf.put_u16(*name_index);
                buf.put_u16(*descriptor_index);
                len += 4;
            }
            Constant::Utf8(string) => {
                buf.put_u8(Tag::Utf8.into());
                len += write_string((*string).clone(), buf);
            }
            Constant::MethodHandle { reference_kind, reference_index } => {
                buf.put_u8(Tag::MethodHandle.into());
                buf.put_u8(*reference_kind);
                buf.put_u16(*reference_index);
                len += 3;
            }
            Constant::MethodType { descriptor_index } => {
                buf.put_u8(Tag::MethodType.into());
                buf.put_u16(*descriptor_index);
                len += 2;
            }
            Constant::InvokeDynamic { bootstrap_method_attr_index, name_and_type_index } => {
                buf.put_u8(Tag::InvokeDynamic.into());
                buf.put_u16(*bootstrap_method_attr_index);
                buf.put_u16(*name_and_type_index);
                len += 4;
            }
        }
        Ok(len)
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