use crate::{FromToBytes, read_string};
use bytes::{BytesMut, BufMut, Buf};
use crate::error::Error;
use crate::ops;

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
    fn to_buf(&self, buf: &mut impl BufMut) -> usize {
        unimplemented!()
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
            },
            ops::CONSTANT_METHOD_REF_TAG => {
                let class_index = buf.get_u16();
                let name_and_type_index = buf.get_u16();
                Ok(Constant::FieldRef { class_index, name_and_type_index })
            },
            ops::CONSTANT_INTERFACE_METHOD_REF_TAG => {
                let class_index = buf.get_u16();
                let name_and_type_index = buf.get_u16();
                Ok(Constant::FieldRef { class_index, name_and_type_index })
            },
            ops::CONSTANT_STRING_TAG => {
                let string_index = buf.get_u16();
                Ok(Constant::String {string_index})
            },
            ops::CONSTANT_INTEGER_TAG => {
                let value = buf.get_i32();
                Ok(Constant::Integer(value))
            },
            ops::CONSTANT_FLOAT_TAG => {
                let value = buf.get_f32();
                Ok(Constant::Float(value))
            },
            ops::CONSTANT_LONG_TAG => {
                let value = buf.get_i64();
                Ok(Constant::Long(value))
            },
            ops::CONSTANT_DOUBLE_TAG => {
                let value = buf.get_f64();
                Ok(Constant::Double(value))
            },
            ops::CONSTANT_NAME_AND_TYPE_TAG => {
                let name_index = buf.get_u16();
                let descriptor_index = buf.get_u16();
                Ok(Constant::NameAndType {name_index, descriptor_index})
            },
            ops::CONSTANT_UTF8_TAG => {
                Ok(Constant::Utf8(read_string(buf)?))
            },
            ops::CONSTANT_METHOD_HANDLE_TAG => {
                let reference_kind = buf.get_u8();
                let reference_index = buf.get_u16();
                Ok(Constant::MethodHandle {reference_kind, reference_index})
            },
            ops::CONSTANT_METHOD_TYPE_TAG => {
                let descriptor_index = buf.get_u16();
                Ok(Constant::MethodType {descriptor_index})
            },
            ops::CONSTANT_INVOKE_DYNAMIC_TAG => {
                let bootstrap_method_attr_index = buf.get_u16();
                let name_and_type_index = buf.get_u16();
                Ok(Constant::InvokeDynamic {bootstrap_method_attr_index, name_and_type_index})
            },
            _ => Err(Error::InvalidConstantTag(tag))
        }
    }

    fn length(&self) -> usize {
        unimplemented!()
    }
}