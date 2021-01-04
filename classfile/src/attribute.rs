#![allow(dead_code)]

use crate::constant::get_utf8;
use crate::error::Error;
use crate::{ConstantPoolRef, TryFromCp, TryInto, BytesRef};
use bytes::{Buf, BufMut, BytesMut};
use std::convert::TryFrom;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Attribute {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub attr_type: AttributeType,
}

impl TryFromCp<&mut BytesMut> for Attribute {
    type Error = Error;

    fn try_from_cp(
        buf: &mut BytesMut,
        constant_pool: &ConstantPoolRef,
    ) -> Result<Self, Self::Error> {
        let attribute_name_index = buf.get_u16();
        let attribute_length = buf.get_u32();
        let attribute_name = &**get_utf8(constant_pool, attribute_name_index as usize);
        let attr_type = match attribute_name.as_slice() {
            b"ConstantValue" => {
                let constant_value_index = buf.get_u16();
                Ok(AttributeType::ConstantValue {
                    constant_value_index,
                })
            }
            b"Code" => Ok(AttributeType::Code {
                code: CodeAttribute::try_from_cp(buf, constant_pool)?,
            }),
            b"StackMapTable" => {
                let number_of_entries = buf.get_u16();
                let mut entries: Vec<StackMap> = vec![];
                for _ in 0..number_of_entries {
                    entries.push(StackMap::try_from(&mut *buf)?);
                }
                Ok(AttributeType::StackMapTable { entries })
            }
            b"Exceptions" => {
                let number_of_exceptions = buf.get_u16();
                let mut exception_index_table: Vec<u16> = vec![];
                for _ in 0..number_of_exceptions {
                    exception_index_table.push(buf.get_u16());
                }
                Ok(AttributeType::Exceptions {
                    exception_index_table,
                })
            }
            b"InnerClass" => {
                let number_of_classes = buf.get_u16();
                let mut classes: Vec<InnerClass> = vec![];
                for _ in 0..number_of_classes {
                    classes.push(InnerClass::try_from(&mut *buf)?);
                }
                Ok(AttributeType::InnerClasses { classes })
            }
            b"EnclosingMethod" => {
                let class_index = buf.get_u16();
                let method_index = buf.get_u16();
                Ok(AttributeType::EnclosingMethod {
                    class_index,
                    method_index,
                })
            }
            b"Synthetic" => Ok(AttributeType::Synthetic),
            b"Signature" => {
                let signature_index = buf.get_u16();
                Ok(AttributeType::Signature { signature_index })
            }
            b"SourceFile" => {
                let sourcefile_index = buf.get_u16();
                Ok(AttributeType::SourceFile { sourcefile_index })
            }
            b"SourceDebugExtension" => {
                let mut debug_extension: Vec<u8> = vec![];
                for _ in 0..attribute_length {
                    debug_extension.push(buf.get_u8());
                }
                Ok(AttributeType::SourceDebugExtension { debug_extension })
            }
            b"LineNumberTable" => {
                let line_number_table_length = buf.get_u16();
                let mut line_number_table: Vec<LineNumber> = vec![];
                for _ in 0..line_number_table_length {
                    line_number_table.push(LineNumber::try_from(&mut *buf)?);
                }
                Ok(AttributeType::LineNumberTable { line_number_table })
            }
            b"LocalVariableTable" => {
                let local_variable_table_length = buf.get_u16();
                let mut local_variable_table: Vec<LocalVariable> = vec![];
                for _ in 0..local_variable_table_length {
                    local_variable_table.push(LocalVariable::try_from(&mut *buf)?);
                }
                Ok(AttributeType::LocalVariableTable {
                    local_variable_table,
                })
            }
            b"LocalVariableTypeTable" => {
                let local_variable_type_table_length = buf.get_u16();
                let mut local_variable_type_table: Vec<LocalVariableType> = vec![];
                for _ in 0..local_variable_type_table_length {
                    local_variable_type_table.push(LocalVariableType::try_from(&mut *buf)?);
                }
                Ok(AttributeType::LocalVariableTypeTable {
                    local_variable_type_table,
                })
            }
            b"Deprecated" => Ok(AttributeType::Deprecated),
            b"RuntimeVisibleAnnotations" => {
                let num_annotations = buf.get_u16();
                let mut annotations: Vec<Annotation> = vec![];
                for _ in 0..num_annotations {
                    annotations.push(Annotation::try_from(&mut *buf)?);
                }
                Ok(AttributeType::RuntimeVisibleAnnotations { annotations })
            }
            b"RuntimeInvisibleAnnotations" => {
                let num_annotations = buf.get_u16();
                let mut annotations: Vec<Annotation> = vec![];
                for _ in 0..num_annotations {
                    annotations.push(Annotation::try_from(&mut *buf)?);
                }
                Ok(AttributeType::RuntimeInvisibleAnnotations { annotations })
            }
            b"RuntimeVisibleParameterAnnotations" => {
                let num_parameters = buf.get_u8();
                let mut parameter_annotations: Vec<ParameterAnnotation> = vec![];
                for _ in 0..num_parameters {
                    parameter_annotations.push(ParameterAnnotation::try_from(&mut *buf)?);
                }
                Ok(AttributeType::RuntimeVisibleParameterAnnotations {
                    parameter_annotations,
                })
            }
            b"RuntimeInvisibleParameterAnnotations" => {
                let num_parameters = buf.get_u8();
                let mut parameter_annotations: Vec<ParameterAnnotation> = vec![];
                for _ in 0..num_parameters {
                    parameter_annotations.push(ParameterAnnotation::try_from(&mut *buf)?);
                }
                Ok(AttributeType::RuntimeInvisibleParameterAnnotations {
                    parameter_annotations,
                })
            }
            b"RuntimeVisibleTypeAnnotations" => {
                let num_annotations = buf.get_u8();
                let mut type_annotations: Vec<TypeAnnotation> = vec![];
                for _ in 0..num_annotations {
                    type_annotations.push(TypeAnnotation::try_from(&mut *buf)?);
                }
                Ok(AttributeType::RuntimeVisibleTypeAnnotations {
                    annotations: type_annotations,
                })
            }
            b"RuntimeInvisibleTypeAnnotations" => {
                let num_annotations = buf.get_u8();
                let mut type_annotations: Vec<TypeAnnotation> = vec![];
                for _ in 0..num_annotations {
                    type_annotations.push(TypeAnnotation::try_from(&mut *buf)?);
                }
                Ok(AttributeType::RuntimeInvisibleTypeAnnotations {
                    annotations: type_annotations,
                })
            }
            b"AnnotationDefault" => Ok(AttributeType::AnnotationDefault {
                default_value: ElementValue::try_from(&mut *buf)?,
            }),
            b"BootstrapMethods" => {
                let num_bootstrap_methods = buf.get_u16();
                let mut bootstrap_methods: Vec<BootstrapMethod> = vec![];
                for _ in 0..num_bootstrap_methods {
                    bootstrap_methods.push(BootstrapMethod::try_from(&mut *buf)?);
                }
                Ok(AttributeType::BootstrapMethods { bootstrap_methods })
            }
            b"MethodParameters" => {
                let parameters_count = buf.get_u8();
                let mut parameters: Vec<MethodParameter> = vec![];
                for _ in 0..parameters_count {
                    parameters.push(MethodParameter::try_from(&mut *buf)?);
                }
                Ok(AttributeType::MethodParameters { parameters })
            }
            _ => Err(Error::InvalidAttributeName(
                String::from_utf8(attribute_name.clone()).unwrap(),
            )),
        }?;
        Ok(Attribute {
            attribute_name_index,
            attribute_length,
            attr_type,
        })
    }
}

impl<T> TryInto<&mut T, usize> for Attribute
where
    T: BufMut,
{
    type Error = Error;

    fn try_into(&self, buf: &mut T) -> Result<usize, Self::Error> {
        let mut len: usize = 0;
        buf.put_u16(self.attribute_name_index);
        buf.put_u32(self.attribute_length);
        len += 6;
        match &self.attr_type {
            AttributeType::ConstantValue {
                constant_value_index,
            } => {
                buf.put_u16(*constant_value_index);
                len += 2;
            }
            AttributeType::Code { code } => {
                len += code.try_into(buf)?;
            }
            AttributeType::StackMapTable { entries } => {
                for stack_map in entries {
                    len += stack_map.try_into(buf)?;
                }
            }
            AttributeType::Exceptions {
                exception_index_table,
            } => {
                for exception_index in exception_index_table {
                    buf.put_u16(*exception_index);
                    len += 2;
                }
            }
            AttributeType::InnerClasses { classes } => {
                for class in classes {
                    len += class.try_into(buf)?;
                }
            }
            AttributeType::EnclosingMethod {
                class_index,
                method_index,
            } => {
                buf.put_u16(*class_index);
                buf.put_u16(*method_index);
                len += 4;
            }
            AttributeType::Synthetic => {}
            AttributeType::Signature { signature_index } => {
                buf.put_u16(*signature_index);
                len += 2;
            }
            AttributeType::SourceFile { sourcefile_index } => {
                buf.put_u16(*sourcefile_index);
                len += 2;
            }
            AttributeType::SourceDebugExtension { debug_extension } => {
                for debug_info in debug_extension {
                    buf.put_u8(*debug_info);
                    len += 1;
                }
            }
            AttributeType::LineNumberTable { line_number_table } => {
                for line_number in line_number_table {
                    len += line_number.try_into(buf)?;
                }
            }
            AttributeType::LocalVariableTable {
                local_variable_table,
            } => {
                for local_variable in local_variable_table {
                    len += local_variable.try_into(buf)?;
                }
            }
            AttributeType::LocalVariableTypeTable {
                local_variable_type_table,
            } => {
                for local_variable_type in local_variable_type_table {
                    len += local_variable_type.try_into(buf)?;
                }
            }
            AttributeType::Deprecated => {}
            AttributeType::RuntimeVisibleAnnotations { annotations } => {
                for annotation in annotations {
                    len += annotation.try_into(buf)?;
                }
            }
            AttributeType::RuntimeInvisibleAnnotations { annotations } => {
                for annotation in annotations {
                    len += annotation.try_into(buf)?;
                }
            }
            AttributeType::RuntimeVisibleParameterAnnotations {
                parameter_annotations,
            } => {
                for parameter_annotation in parameter_annotations {
                    len += parameter_annotation.try_into(buf)?;
                }
            }
            AttributeType::RuntimeInvisibleParameterAnnotations {
                parameter_annotations,
            } => {
                for parameter_annotation in parameter_annotations {
                    len += parameter_annotation.try_into(buf)?;
                }
            }
            AttributeType::RuntimeVisibleTypeAnnotations { annotations } => {
                for annotation in annotations {
                    len += annotation.try_into(buf)?;
                }
            }
            AttributeType::RuntimeInvisibleTypeAnnotations { annotations } => {
                for annotation in annotations {
                    len += annotation.try_into(buf)?;
                }
            }
            AttributeType::AnnotationDefault { default_value } => {
                len += default_value.try_into(buf)?;
            }
            AttributeType::BootstrapMethods { bootstrap_methods } => {
                for bootstrap_method in bootstrap_methods {
                    len += bootstrap_method.try_into(buf)?;
                }
            }
            AttributeType::MethodParameters { parameters } => {
                for parameter in parameters {
                    len += parameter.try_into(buf)?;
                }
            }
        }
        Ok(len)
    }
}

#[derive(Debug, Clone)]
pub enum AttributeType {
    ConstantValue {
        constant_value_index: u16,
    },
    Code {
        code: CodeAttribute,
    },
    StackMapTable {
        entries: Vec<StackMap>,
    },
    Exceptions {
        exception_index_table: Vec<u16>,
    },
    InnerClasses {
        classes: Vec<InnerClass>,
    },
    EnclosingMethod {
        class_index: u16,
        method_index: u16,
    },
    Synthetic,
    Signature {
        signature_index: u16,
    },
    SourceFile {
        sourcefile_index: u16,
    },
    SourceDebugExtension {
        debug_extension: Vec<u8>,
    },
    LineNumberTable {
        line_number_table: Vec<LineNumber>,
    },
    LocalVariableTable {
        local_variable_table: Vec<LocalVariable>,
    },
    LocalVariableTypeTable {
        local_variable_type_table: Vec<LocalVariableType>,
    },
    Deprecated,
    RuntimeVisibleAnnotations {
        annotations: Vec<Annotation>,
    },
    RuntimeInvisibleAnnotations {
        annotations: Vec<Annotation>,
    },
    RuntimeVisibleParameterAnnotations {
        parameter_annotations: Vec<ParameterAnnotation>,
    },
    RuntimeInvisibleParameterAnnotations {
        parameter_annotations: Vec<ParameterAnnotation>,
    },
    RuntimeVisibleTypeAnnotations {
        annotations: Vec<TypeAnnotation>,
    },
    RuntimeInvisibleTypeAnnotations {
        annotations: Vec<TypeAnnotation>,
    },
    AnnotationDefault {
        default_value: ElementValue,
    },
    BootstrapMethods {
        bootstrap_methods: Vec<BootstrapMethod>,
    },
    MethodParameters {
        parameters: Vec<MethodParameter>,
    },
}

#[derive(Debug, Clone)]
pub struct CodeAttribute {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code:BytesRef,
    pub exception_table: Vec<Exception>,
    pub attributes: Vec<Attribute>,
}

impl TryFromCp<&mut BytesMut> for CodeAttribute {
    type Error = Error;

    fn try_from_cp(
        buf: &mut BytesMut,
        constant_pool: &ConstantPoolRef,
    ) -> Result<Self, Self::Error> {
        let max_stack = buf.get_u16();
        let max_locals = buf.get_u16();
        let code_length = buf.get_u32();
        let mut code: Vec<u8> = vec![];
        for _ in 0..code_length {
            code.push(buf.get_u8());
        }
        let exception_table_length = buf.get_u16();
        let mut exception_table: Vec<Exception> = vec![];
        for _ in 0..exception_table_length {
            exception_table.push(Exception::try_from(&mut *buf)?);
        }
        let attributes_count = buf.get_u16();
        let mut attributes: Vec<Attribute> = vec![];
        for _ in 0..attributes_count {
            attributes.push(Attribute::try_from_cp(buf, constant_pool)?);
        }
        Ok(CodeAttribute {
            max_stack,
            max_locals,
            code: Arc::new(code),
            exception_table,
            attributes,
        })
    }
}

impl<T> TryInto<&mut T, usize> for CodeAttribute
where
    T: BufMut,
{
    type Error = Error;

    fn try_into(&self, buf: &mut T) -> Result<usize, Self::Error> {
        let mut len: usize = 0;
        buf.put_u16(self.max_stack);
        buf.put_u16(self.max_locals);
        let code_length = self.code.len() as u32;
        buf.put_u32(code_length);
        len += 8;
        for byte in &*self.code {
            buf.put_u8(*byte);
            len += 1;
        }
        for exception in &self.exception_table {
            len += exception.try_into(buf)?;
        }
        let attribute_count = self.attributes.len() as u16;
        buf.put_u16(attribute_count);
        len += 2;
        for attribute in &self.attributes {
            len += attribute.try_into(buf)?;
        }
        Ok(len)
    }
}

#[derive(Debug, Clone)]
pub struct Exception {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

impl TryFrom<&mut BytesMut> for Exception {
    type Error = Error;

    fn try_from(buf: &mut BytesMut) -> Result<Self, Self::Error> {
        let start_pc = buf.get_u16();
        let end_pc = buf.get_u16();
        let handler_pc = buf.get_u16();
        let catch_type = buf.get_u16();
        Ok(Exception {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        })
    }
}

impl<T> TryInto<&mut T, usize> for Exception
where
    T: BufMut,
{
    type Error = Error;

    fn try_into(&self, buf: &mut T) -> Result<usize, Self::Error> {
        let mut len: usize = 0;
        buf.put_u16(self.start_pc);
        buf.put_u16(self.end_pc);
        buf.put_u16(self.handler_pc);
        buf.put_u16(self.catch_type);
        len += 8;
        Ok(len)
    }
}

#[derive(Debug, Clone)]
pub enum StackMapFrame {
    SameFrame,
    SameLocals1StackItemFrame {
        stack: VerificationTypeInfo,
    },
    SameLocals1StackItemFrameExtended {
        offset_delta: u16,
        stack: VerificationTypeInfo,
    },
    ChopFrame {
        offset_delta: u16,
    },
    SameFrameExtended {
        offset_delta: u16,
    },
    AppendFrame {
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
    },
    FullFrame {
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
        stack: Vec<VerificationTypeInfo>,
    },
}

#[derive(Debug, Clone)]
pub struct StackMap {
    frame_type: u8,
    frame: StackMapFrame,
}

impl TryFrom<&mut BytesMut> for StackMap {
    type Error = Error;

    fn try_from(buf: &mut BytesMut) -> Result<Self, Self::Error> {
        let frame_type = buf.get_u8();
        let frame: StackMapFrame;
        match frame_type {
            0..=63 => {
                frame = StackMapFrame::SameFrame;
            }
            64..=127 => {
                let stack = VerificationTypeInfo::try_from(buf)?;
                frame = StackMapFrame::SameLocals1StackItemFrame { stack };
            }
            247 => {
                let offset_delta = buf.get_u16();
                let stack = VerificationTypeInfo::try_from(buf)?;
                frame = StackMapFrame::SameLocals1StackItemFrameExtended {
                    offset_delta,
                    stack,
                };
            }
            248..=250 => {
                let offset_delta = buf.get_u16();
                frame = StackMapFrame::ChopFrame { offset_delta };
            }
            251 => {
                let offset_delta = buf.get_u16();
                frame = StackMapFrame::SameFrameExtended { offset_delta };
            }
            252..=254 => {
                let offset_delta = buf.get_u16();
                let frame_type = frame_type;
                let num_verification_type_info = frame_type - 251;
                let mut locals: Vec<VerificationTypeInfo> = vec![];
                for _ in 0..num_verification_type_info {
                    let local = VerificationTypeInfo::try_from(&mut *buf)?;
                    locals.push(local);
                }
                frame = StackMapFrame::AppendFrame {
                    offset_delta,
                    locals,
                };
            }
            255 => {
                let offset_delta = buf.get_u16();
                let number_of_locals = buf.get_u16();
                let mut locals: Vec<VerificationTypeInfo> = vec![];
                for _ in 0..number_of_locals {
                    let local = VerificationTypeInfo::try_from(&mut *buf)?;
                    locals.push(local);
                }
                let number_of_stack_items = buf.get_u16();
                let mut stack: Vec<VerificationTypeInfo> = vec![];
                for _ in 0..number_of_stack_items {
                    let stack_item = VerificationTypeInfo::try_from(&mut *buf)?;
                    stack.push(stack_item);
                }
                frame = StackMapFrame::FullFrame {
                    offset_delta,
                    locals,
                    stack,
                };
            }
            _ => {
                return Err(Error::InvalidFrameType);
            }
        }
        Ok(StackMap { frame_type, frame })
    }
}

impl<T> TryInto<&mut T, usize> for StackMap
where
    T: BufMut,
{
    type Error = Error;

    fn try_into(&self, buf: &mut T) -> Result<usize, Self::Error> {
        let mut len: usize = 0;
        buf.put_u8(self.frame_type);
        len += 1;
        match self.frame_type {
            0..=63 => {}
            64..=127 => {
                if let StackMapFrame::SameLocals1StackItemFrame { stack } = &self.frame {
                    len += stack.try_into(buf)?;
                } else {
                    return Err(Error::MismatchFrameType(
                        self.frame_type,
                        self.frame.clone(),
                    ));
                }
            }
            247 => {
                if let StackMapFrame::SameLocals1StackItemFrameExtended {
                    offset_delta,
                    stack,
                } = &self.frame
                {
                    buf.put_u16(*offset_delta);
                    len += 2;
                    len += stack.try_into(buf)?;
                } else {
                    return Err(Error::MismatchFrameType(
                        self.frame_type,
                        self.frame.clone(),
                    ));
                }
            }
            248..=250 => {
                if let StackMapFrame::ChopFrame { offset_delta } = self.frame {
                    buf.put_u16(offset_delta);
                    len += 2;
                } else {
                    return Err(Error::MismatchFrameType(
                        self.frame_type,
                        self.frame.clone(),
                    ));
                }
            }
            251 => {
                if let StackMapFrame::SameFrameExtended { offset_delta } = self.frame {
                    buf.put_u16(offset_delta);
                    len += 2;
                } else {
                    return Err(Error::MismatchFrameType(
                        self.frame_type,
                        self.frame.clone(),
                    ));
                }
            }
            252..=254 => {
                if let StackMapFrame::AppendFrame {
                    offset_delta,
                    locals,
                } = &self.frame
                {
                    buf.put_u16(*offset_delta);
                    len += 2;
                    for verification_type_info in locals {
                        len += verification_type_info.try_into(buf)?;
                    }
                } else {
                    return Err(Error::MismatchFrameType(
                        self.frame_type,
                        self.frame.clone(),
                    ));
                }
            }
            255 => {
                if let StackMapFrame::FullFrame {
                    offset_delta,
                    locals,
                    stack,
                } = &self.frame
                {
                    buf.put_u16(*offset_delta);
                    for verification_type_info in locals {
                        len += verification_type_info.try_into(buf)?;
                    }
                    for verification_type_info in stack {
                        len += verification_type_info.try_into(buf)?;
                    }
                } else {
                    return Err(Error::MismatchFrameType(
                        self.frame_type,
                        self.frame.clone(),
                    ));
                }
            }
            _ => {
                return Err(Error::InvalidFrameType);
            }
        }
        Ok(len)
    }
}

#[derive(Debug, Clone)]
pub enum VerificationTypeInfo {
    Top,
    Integer,
    Float,
    Long,
    Double,
    Null,
    UninitializedThis,
    Object { cpool_index: u16 },
    Uninitialized { offset: u16 },
}

impl TryFrom<&mut BytesMut> for VerificationTypeInfo {
    type Error = Error;

    fn try_from(buf: &mut BytesMut) -> Result<Self, Self::Error> {
        let tag = buf.get_u8();
        return match tag {
            0 => Ok(VerificationTypeInfo::Top),
            1 => Ok(VerificationTypeInfo::Integer),
            2 => Ok(VerificationTypeInfo::Float),
            3 => Ok(VerificationTypeInfo::Double),
            4 => Ok(VerificationTypeInfo::Long),
            5 => Ok(VerificationTypeInfo::Null),
            6 => Ok(VerificationTypeInfo::UninitializedThis),
            7 => {
                let cpool_index = buf.get_u16();
                Ok(VerificationTypeInfo::Object { cpool_index })
            }
            8 => {
                let offset = buf.get_u16();
                Ok(VerificationTypeInfo::Uninitialized { offset })
            }
            _ => Err(Error::InvalidVerificationTypeInfo),
        };
    }
}

impl<T> TryInto<&mut T, usize> for VerificationTypeInfo
where
    T: BufMut,
{
    type Error = Error;

    fn try_into(&self, buf: &mut T) -> Result<usize, Self::Error> {
        let mut len: usize = 1;
        match self {
            VerificationTypeInfo::Top => {
                buf.put_u8(0u8);
            }
            VerificationTypeInfo::Integer => {
                buf.put_u8(1u8);
            }
            VerificationTypeInfo::Float => {
                buf.put_u8(2u8);
            }
            VerificationTypeInfo::Double => {
                buf.put_u8(3u8);
            }
            VerificationTypeInfo::Long => {
                buf.put_u8(4u8);
            }
            VerificationTypeInfo::Null => {
                buf.put_u8(5u8);
            }
            VerificationTypeInfo::UninitializedThis => {
                buf.put_u8(6u8);
            }
            VerificationTypeInfo::Object { cpool_index } => {
                buf.put_u8(7u8);
                buf.put_u16(*cpool_index);
                len += 2;
            }
            VerificationTypeInfo::Uninitialized { offset } => {
                buf.put_u8(8u8);
                buf.put_u16(*offset);
                len += 2;
            }
        }
        Ok(len)
    }
}

#[derive(Debug, Clone)]
pub struct InnerClass {
    pub inner_class_info_index: u16,
    pub outer_class_info_index: u16,
    pub inner_name_index: u16,
    pub inner_class_access_flags: u16,
}

impl TryFrom<&mut BytesMut> for InnerClass {
    type Error = Error;

    fn try_from(buf: &mut BytesMut) -> Result<Self, Self::Error> {
        let inner_class_info_index = buf.get_u16();
        let outer_class_info_index = buf.get_u16();
        let inner_name_index = buf.get_u16();
        let inner_class_access_flags = buf.get_u16();
        Ok(InnerClass {
            inner_class_info_index,
            outer_class_info_index,
            inner_name_index,
            inner_class_access_flags,
        })
    }
}

impl<T> TryInto<&mut T, usize> for InnerClass
where
    T: BufMut,
{
    type Error = Error;

    fn try_into(&self, buf: &mut T) -> Result<usize, Self::Error> {
        let mut len: usize = 0;
        buf.put_u16(self.inner_class_info_index);
        buf.put_u16(self.outer_class_info_index);
        buf.put_u16(self.inner_name_index);
        buf.put_u16(self.inner_class_access_flags);
        len += 8;
        Ok(len)
    }
}

#[derive(Debug, Clone)]
pub struct LineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}

impl TryFrom<&mut BytesMut> for LineNumber {
    type Error = Error;

    fn try_from(buf: &mut BytesMut) -> Result<Self, Self::Error> {
        let start_pc = buf.get_u16();
        let line_number = buf.get_u16();
        Ok(LineNumber {
            start_pc,
            line_number,
        })
    }
}

impl<T> TryInto<&mut T, usize> for LineNumber
where
    T: BufMut,
{
    type Error = Error;

    fn try_into(&self, buf: &mut T) -> Result<usize, Self::Error> {
        let mut len: usize = 0;
        buf.put_u16(self.start_pc);
        buf.put_u16(self.line_number);
        len += 4;
        Ok(len)
    }
}

#[derive(Debug, Clone)]
pub struct LocalVariable {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

impl TryFrom<&mut BytesMut> for LocalVariable {
    type Error = Error;

    fn try_from(buf: &mut BytesMut) -> Result<Self, Self::Error> {
        let start_pc = buf.get_u16();
        let length = buf.get_u16();
        let name_index = buf.get_u16();
        let descriptor_index = buf.get_u16();
        let index = buf.get_u16();
        Ok(LocalVariable {
            start_pc,
            length,
            name_index,
            descriptor_index,
            index,
        })
    }
}

impl<T> TryInto<&mut T, usize> for LocalVariable
where
    T: BufMut,
{
    type Error = Error;

    fn try_into(&self, buf: &mut T) -> Result<usize, Self::Error> {
        let mut len: usize = 0;
        buf.put_u16(self.start_pc);
        buf.put_u16(self.length);
        buf.put_u16(self.name_index);
        buf.put_u16(self.descriptor_index);
        buf.put_u16(self.index);
        len += 10;
        Ok(len)
    }
}

#[derive(Debug, Clone)]
pub struct LocalVariableType {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub signature_index: u16,
    pub index: u16,
}

impl TryFrom<&mut BytesMut> for LocalVariableType {
    type Error = Error;

    fn try_from(buf: &mut BytesMut) -> Result<Self, Self::Error> {
        let start_pc = buf.get_u16();
        let length = buf.get_u16();
        let name_index = buf.get_u16();
        let signature_index = buf.get_u16();
        let index = buf.get_u16();
        Ok(LocalVariableType {
            start_pc,
            length,
            name_index,
            signature_index,
            index,
        })
    }
}

impl<T> TryInto<&mut T, usize> for LocalVariableType
where
    T: BufMut,
{
    type Error = Error;

    fn try_into(&self, buf: &mut T) -> Result<usize, Self::Error> {
        let mut len: usize = 0;
        buf.put_u16(self.start_pc);
        buf.put_u16(self.length);
        buf.put_u16(self.name_index);
        buf.put_u16(self.signature_index);
        buf.put_u16(self.index);
        len += 10;
        Ok(len)
    }
}

///```jvm
/// annotation {
///     u2 type_index;
///     u2 num_element_value_pairs;
///     {
///         u2 element_name_index;
///         element_value value;
///     } element_value_pairs[num_element_value_pairs];
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Annotation {
    pub type_index: u16,
    /// 0. element_name_index
    /// 1. value
    pub element_value_pairs: Vec<(u16, ElementValue)>,
}

impl TryFrom<&mut BytesMut> for Annotation {
    type Error = Error;

    fn try_from(buf: &mut BytesMut) -> Result<Self, Self::Error> {
        let type_index = buf.get_u16();
        let num_element_value_pairs = buf.get_u16();
        let mut element_value_pairs: Vec<(u16, ElementValue)> = vec![];
        for _ in 0..num_element_value_pairs {
            let element_name_index = buf.get_u16();
            let element_value = ElementValue::try_from(&mut *buf)?;
            element_value_pairs.push((element_name_index, element_value));
        }
        Ok(Annotation {
            type_index,
            element_value_pairs,
        })
    }
}

impl<T> TryInto<&mut T, usize> for Annotation
where
    T: BufMut,
{
    type Error = Error;

    fn try_into(&self, buf: &mut T) -> Result<usize, Self::Error> {
        let mut len: usize = 0;
        buf.put_u16(self.type_index);
        len += 2;
        for element_value_pair in &self.element_value_pairs {
            buf.put_u16(element_value_pair.0);
            len += 2;
            len += element_value_pair.1.try_into(buf)?;
        }
        Ok(len)
    }
}

///```jvm
/// element_value {
///     u1 tag;
///     union {
///         u2 const_value_index;
///         {
///             u2 type_name_index;
///             u2 const_name_index;
///         } enum_const_value;
///         u2 class_info_index;
///         annotation annotation_value;
///         {
///             u2 num_values;
///             element_value values[num_values];
///         } array_value;
///     } value;
/// }
///```
///
/// |tag Item|Type|value Item|Constant Type|
/// |---|---|---|---|
/// |B|byte|const_value_index|CONSTANT_Integer|
/// |C|char|const_value_index|CONSTANT_Integer|
/// |D|double|const_value_index|CONSTANT_Double|
/// |F|float|const_value_index|CONSTANT_Float|
/// |I|int|const_value_index|CONSTANT_Integer|
/// |J|long|const_value_index|CONSTANT_Long|
/// |S|short|const_value_index|CONSTANT_Integer|
/// |Z|boolean|const_value_index|CONSTANT_Integer|
/// |s|String|const_value_index|CONSTANT_Utf8|
/// |e|Enum Type|enum_const_value|Not applicable|
/// |c|Class|class_info_index|Not applicable|
/// |@|Annotation Type|annotation_value|Not applicable|
/// |[|Array type|array_value|Not applicable|
#[derive(Debug, Clone)]
pub struct ElementValue {
    pub tag: u8,
    pub value: Element,
}

#[derive(Debug, Clone)]
pub enum Element {
    ConstValueIndex(u16),
    /// 0. type_name_index
    /// 1. const_name_index
    EnumConstValue((u16, u16)),
    ClassInfoIndex(u16),
    AnnotationValue(Annotation),
    ArrayValue(Vec<ElementValue>),
}

impl TryFrom<&mut BytesMut> for ElementValue {
    type Error = Error;

    fn try_from(buf: &mut BytesMut) -> Result<Self, Self::Error> {
        let tag = buf.get_u8() as char;
        let value: Element;
        match tag {
            'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z' | 's' => {
                value = Element::ConstValueIndex(buf.get_u16());
            }
            'e' => {
                let type_name_index = buf.get_u16();
                let const_name_index = buf.get_u16();
                value = Element::EnumConstValue((type_name_index, const_name_index));
            }
            'c' => {
                value = Element::ClassInfoIndex(buf.get_u16());
            }
            '@' => {
                value = Element::AnnotationValue(Annotation::try_from(buf)?);
            }
            '[' => {
                let num_values = buf.get_u16();
                let mut values: Vec<ElementValue> = vec![];
                for _ in 0..num_values {
                    values.push(ElementValue::try_from(&mut *buf)?);
                }
                value = Element::ArrayValue(values);
            }
            c => {
                return Err(Error::InvalidElementValueTag(c));
            }
        }
        let tag = tag as u8;
        Ok(ElementValue { tag, value })
    }
}

impl<T> TryInto<&mut T, usize> for ElementValue
where
    T: BufMut,
{
    type Error = Error;

    fn try_into(&self, buf: &mut T) -> Result<usize, Self::Error> {
        let mut len: usize = 0;
        let tag = self.tag as char;
        match tag {
            'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z' | 's' => {
                if let Element::ConstValueIndex(const_value_index) = self.value {
                    buf.put_u16(const_value_index);
                    len += 2;
                } else {
                    return Err(Error::InvalidElementValue);
                }
            }
            'e' => {
                if let Element::EnumConstValue((type_name_index, const_name_index)) = self.value {
                    buf.put_u16(type_name_index);
                    buf.put_u16(const_name_index);
                    len += 4;
                } else {
                    return Err(Error::InvalidElementValue);
                }
            }
            'c' => {
                if let Element::ClassInfoIndex(class_info_index) = self.value {
                    buf.put_u16(class_info_index);
                    len += 2;
                } else {
                    return Err(Error::InvalidElementValue);
                }
            }
            '@' => {
                if let Element::AnnotationValue(annotation) = &self.value {
                    len += annotation.try_into(buf)?;
                } else {
                    return Err(Error::InvalidElementValue);
                }
            }
            '[' => {
                if let Element::ArrayValue(values) = &self.value {
                    let num_values = values.len() as u16;
                    buf.put_u16(num_values);
                    len += 2;
                    for element_value in values {
                        len += element_value.try_into(buf)?;
                    }
                } else {
                    return Err(Error::InvalidElementValue);
                }
            }
            c => {
                return Err(Error::InvalidElementValueTag(c));
            }
        }
        Ok(len)
    }
}

#[derive(Debug, Clone)]
pub struct ParameterAnnotation {
    annotations: Vec<Annotation>,
}

impl TryFrom<&mut BytesMut> for ParameterAnnotation {
    type Error = Error;

    fn try_from(buf: &mut BytesMut) -> Result<Self, Self::Error> {
        let num_annotations = buf.get_u16();
        let mut annotations: Vec<Annotation> = vec![];
        for _ in 0..num_annotations {
            annotations.push(Annotation::try_from(&mut *buf)?);
        }
        Ok(ParameterAnnotation { annotations })
    }
}

impl<T> TryInto<&mut T, usize> for ParameterAnnotation
where
    T: BufMut,
{
    type Error = Error;

    fn try_into(&self, buf: &mut T) -> Result<usize, Self::Error> {
        let mut len: usize = 0;
        buf.put_u16(self.annotations.len() as u16);
        for annotation in &self.annotations {
            len += annotation.try_into(buf)?;
        }
        Ok(len)
    }
}

///```jvm
/// type_annotation {
///     u1 target_type;
///     union {
///         type_parameter_target;
///         supertype_target;
///         type_parameter_bound_target;
///         empty_target;
///         method_formal_parameter_target;
///         throws_target;
///         localvar_target;
///         catch_target;
///         offset_target;
///         type_argument_target;
///     } target_info;
///     type_path target_path;
///     u2 type_index;
///     u2 num_element_value_pairs;
///     {
///         u2 element_name_index;
///         element_value value;
///     } element_value_pairs[num_element_value_pairs];
/// }
///```
///
/// ### Interpretation of target_type values (Part 1)
///
/// | Value | Kind of target | target_info Item |
/// |---|---|---|
/// |0x00|type parameter declaration of generic class or interface|type_parameter_target|
/// |0x01|
/// TODO
#[derive(Debug, Clone)]
pub struct TypeAnnotation {
    pub target_type: u8,
    pub target_info: TargetInfo,
    pub type_path: TypePath,
    pub type_index: u16,
    pub element_value_pairs: Vec<(u16, ElementValue)>,
}

#[derive(Debug, Clone)]
pub enum TargetInfo {
    /// type_parameter_index
    TypeParameterTarget(u8),
    /// supertype_index
    SupertypeTarget(u16),
    TypeParameterBoundTarget {
        type_parameter_index: u8,
        bound_index: u8,
    },
    EmptyTarget,
    /// formal_parameter_index
    FormalParameterTarget(u8),
    /// throws_type_index
    ThrowTarget(u16),
    LocalVarTarget(Vec<LocalVar>),
    /// exception_table_index
    CatchTarget(u16),
    /// offset
    OffsetTarget(u16),
    TypeArgumentTarget {
        offset: u16,
        type_argument_index: u8,
    },
}

impl TryFrom<&mut BytesMut> for TypeAnnotation {
    type Error = Error;

    fn try_from(buf: &mut BytesMut) -> Result<Self, Self::Error> {
        let target_type = buf.get_u8();
        let target_info: TargetInfo;
        match target_type {
            0x00 | 0x01 => {
                let type_parameter_index = buf.get_u8();
                target_info = TargetInfo::TypeParameterTarget(type_parameter_index);
            }
            0x10 => {
                let supertype_index = buf.get_u16();
                target_info = TargetInfo::SupertypeTarget(supertype_index);
            }
            0x11 | 0x12 => {
                let type_parameter_index = buf.get_u8();
                let bound_index = buf.get_u8();
                target_info = TargetInfo::TypeParameterBoundTarget {
                    type_parameter_index,
                    bound_index,
                };
            }
            0x13 | 0x14 | 0x15 => target_info = TargetInfo::EmptyTarget,
            0x16 => {
                let formal_parameter_index = buf.get_u8();
                target_info = TargetInfo::FormalParameterTarget(formal_parameter_index);
            }
            0x17 => {
                let throws_type_index = buf.get_u16();
                target_info = TargetInfo::ThrowTarget(throws_type_index);
            }
            0x40 | 0x41 => {
                let table_length = buf.get_u16();
                let mut local_vars: Vec<LocalVar> = vec![];
                for _ in 0..table_length {
                    let local_var = match LocalVar::try_from(&mut *buf) {
                        Ok(local_var) => local_var,
                        Err(e) => return Err(e),
                    };
                    local_vars.push(local_var);
                }
                target_info = TargetInfo::LocalVarTarget(local_vars);
            }
            0x42 => {
                let exception_table_index = buf.get_u16();
                target_info = TargetInfo::CatchTarget(exception_table_index);
            }
            0x43 | 0x44 | 0x45 | 0x46 => {
                let offset = buf.get_u16();
                target_info = TargetInfo::OffsetTarget(offset);
            }
            0x47 | 0x48 | 0x49 | 0x4A | 0x4B => {
                let offset = buf.get_u16();
                let type_argument_index = buf.get_u8();
                target_info = TargetInfo::TypeArgumentTarget {
                    offset,
                    type_argument_index,
                };
            }
            _ => {
                return Err(Error::InvalidTargetType(target_type));
            }
        }
        let target_path = TypePath::try_from(&mut *buf).unwrap();
        let type_index = buf.get_u16();
        let num_element_value_pairs = buf.get_u16();
        let mut element_value_pairs: Vec<(u16, ElementValue)> = vec![];
        for _ in 0..num_element_value_pairs {
            let element_name_index = buf.get_u16();
            let value = match ElementValue::try_from(&mut *buf) {
                Ok(value) => value,
                Err(e) => return Err(e),
            };
            element_value_pairs.push((element_name_index, value))
        }
        Ok(TypeAnnotation {
            target_type,
            target_info,
            type_path: target_path,
            type_index,
            element_value_pairs,
        })
    }
}

impl<T> TryInto<&mut T, usize> for TypeAnnotation
where
    T: BufMut,
{
    type Error = Error;

    fn try_into(&self, buf: &mut T) -> Result<usize, Self::Error> {
        let mut len: usize = 0;
        buf.put_u8(self.target_type);
        len += 1;
        match self.target_type {
            0x00 | 0x01 => {
                if let TargetInfo::TypeParameterTarget(type_parameter_index) = self.target_info {
                    buf.put_u8(type_parameter_index);
                    len += 1;
                } else {
                    return Err(Error::InvalidTargetInfo);
                }
            }
            0x10 => {
                if let TargetInfo::SupertypeTarget(supertype_index) = self.target_info {
                    buf.put_u16(supertype_index);
                    len += 2;
                } else {
                    return Err(Error::InvalidTargetInfo);
                }
            }
            0x11 | 0x12 => {
                if let TargetInfo::TypeParameterBoundTarget {
                    type_parameter_index,
                    bound_index,
                } = self.target_info
                {
                    buf.put_u8(type_parameter_index);
                    buf.put_u8(bound_index);
                    len += 2;
                } else {
                    return Err(Error::InvalidTargetInfo);
                }
            }
            0x13 | 0x14 | 0x15 => {
                // EmptyTarget
                log::info!("Nothing parsed from TargetInfo::EmptyTarget");
            }
            0x16 => {
                if let TargetInfo::FormalParameterTarget(formal_parameter_index) = self.target_info
                {
                    buf.put_u8(formal_parameter_index);
                    len += 1;
                } else {
                    return Err(Error::InvalidTargetInfo);
                }
            }
            0x17 => {
                if let TargetInfo::ThrowTarget(throws_type_index) = self.target_info {
                    buf.put_u16(throws_type_index);
                    len += 2;
                } else {
                    return Err(Error::InvalidTargetInfo);
                }
            }
            0x40 | 0x41 => {
                if let TargetInfo::LocalVarTarget(local_vars) = &self.target_info {
                    buf.put_u16(local_vars.len() as u16);
                    len += 2;
                    for i in 0..local_vars.len() {
                        len += local_vars[i].try_into(buf)?;
                    }
                } else {
                    return Err(Error::InvalidTargetInfo);
                }
            }
            0x42 => {
                if let TargetInfo::CatchTarget(exception_table_index) = self.target_info {
                    buf.put_u16(exception_table_index);
                    len += 2;
                } else {
                    return Err(Error::InvalidTargetInfo);
                }
            }
            0x43 | 0x44 | 0x45 | 0x46 => {
                if let TargetInfo::OffsetTarget(offset) = self.target_info {
                    buf.put_u16(offset);
                    len += 2;
                } else {
                    return Err(Error::InvalidTargetInfo);
                }
            }
            0x47 | 0x48 | 0x49 | 0x4A | 0x4B => {
                if let TargetInfo::TypeArgumentTarget {
                    offset,
                    type_argument_index,
                } = self.target_info
                {
                    buf.put_u16(offset);
                    buf.put_u8(type_argument_index);
                    len += 3;
                } else {
                    return Err(Error::InvalidTargetInfo);
                }
            }
            _ => {
                log::error!("Invalid target type: {}", self.target_type);
                return Err(Error::InvalidTargetType(self.target_type));
            }
        }
        len += self.type_path.try_into(buf)?;
        buf.put_u16(self.type_index);
        buf.put_u16(self.element_value_pairs.len() as u16);
        len += 4;
        for element_value_pair in &self.element_value_pairs {
            buf.put_u16(element_value_pair.0);
            len += 1;
            len += element_value_pair.1.try_into(buf)?;
        }
        Ok(len)
    }
}

/// ```jvm
/// type_path {
///     u1 path_length;
///     {
///         u1 type_path_kind;
///         u1 type_argument_index;
///     } path[path_length];
/// }
/// ```
#[derive(Debug, Clone)]
pub struct TypePath {
    /// 0. type_path_kind
    /// 1. type_argument_index
    pub path: Vec<(u8, u8)>,
}

impl TryFrom<&mut BytesMut> for TypePath {
    type Error = Error;

    fn try_from(buf: &mut BytesMut) -> Result<Self, Self::Error> {
        let path_length = buf.get_u8();
        let mut path: Vec<(u8, u8)> = vec![];
        for _ in 0..path_length {
            let type_path_kind = buf.get_u8();
            let type_argument_index = buf.get_u8();
            path.push((type_path_kind, type_argument_index));
        }
        Ok(TypePath { path })
    }
}

impl<T> TryInto<&mut T, usize> for TypePath
where
    T: BufMut,
{
    type Error = Error;

    fn try_into(&self, buf: &mut T) -> Result<usize, Self::Error> {
        let mut len: usize = 0;
        let path_length = self.path.len() as u8;
        buf.put_u8(path_length);
        len += 1;
        for i in 0..(path_length as usize) {
            buf.put_u8(self.path[i].0);
            buf.put_u8(self.path[i].1);
            len += 2;
        }
        Ok(len)
    }
}

#[derive(Debug, Clone)]
pub struct LocalVar {
    pub start_pc: u16,
    pub length: u16,
    pub index: u16,
}

impl TryFrom<&mut BytesMut> for LocalVar {
    type Error = Error;

    fn try_from(buf: &mut BytesMut) -> Result<Self, Self::Error> {
        let start_pc = buf.get_u16();
        let length = buf.get_u16();
        let index = buf.get_u16();
        Ok(LocalVar {
            start_pc,
            length,
            index,
        })
    }
}

impl<T> TryInto<&mut T, usize> for LocalVar
where
    T: BufMut,
{
    type Error = Error;

    fn try_into(&self, buf: &mut T) -> Result<usize, Self::Error> {
        let mut len: usize = 0;
        buf.put_u16(self.start_pc);
        buf.put_u16(self.length);
        buf.put_u16(self.index);
        len += 6;
        Ok(len)
    }
}

#[derive(Debug, Clone)]
pub struct BootstrapMethod {
    pub bootstrap_method_ref: u16,
    pub bootstrap_arguments: Vec<u16>,
}

impl TryFrom<&mut BytesMut> for BootstrapMethod {
    type Error = Error;

    fn try_from(buf: &mut BytesMut) -> Result<Self, Self::Error> {
        let bootstrap_method_ref = buf.get_u16();
        let num_bootstrap_arguments = buf.get_u16();
        let mut bootstrap_arguments: Vec<u16> = vec![];
        for _ in 0..num_bootstrap_arguments {
            bootstrap_arguments.push(buf.get_u16());
        }
        Ok(BootstrapMethod {
            bootstrap_method_ref,
            bootstrap_arguments,
        })
    }
}

impl<T> TryInto<&mut T, usize> for BootstrapMethod
where
    T: BufMut,
{
    type Error = Error;

    fn try_into(&self, buf: &mut T) -> Result<usize, Self::Error> {
        let mut len: usize = 0;
        buf.put_u16(self.bootstrap_method_ref);
        len += 2;
        let num_bootstrap_arguments = self.bootstrap_arguments.len() as u16;
        buf.put_u16(num_bootstrap_arguments);
        len += 2;
        for i in 0..self.bootstrap_arguments.len() {
            buf.put_u16(self.bootstrap_arguments[i]);
            len += 2;
        }
        Ok(len)
    }
}

#[derive(Debug, Clone)]
pub struct MethodParameter {
    pub name_index: u16,
    pub access_flags: u16,
}

impl TryFrom<&mut BytesMut> for MethodParameter {
    type Error = Error;

    fn try_from(buf: &mut BytesMut) -> Result<Self, Self::Error> {
        let name_index = buf.get_u16();
        let access_flags = buf.get_u16();
        Ok(MethodParameter {
            name_index,
            access_flags,
        })
    }
}

impl<T> TryInto<&mut T, usize> for MethodParameter
where
    T: BufMut,
{
    type Error = Error;

    fn try_into(&self, buf: &mut T) -> Result<usize, Self::Error> {
        let mut len: usize = 0;
        buf.put_u16(self.name_index);
        buf.put_u16(self.access_flags);
        len += 4;
        Ok(len)
    }
}
