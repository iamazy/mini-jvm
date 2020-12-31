use std::fmt::Formatter;
#[allow(dead_code)]
use std::fmt::{self, Display};

#[derive(Debug, Clone)]
pub enum FieldType<'a> {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Reference(&'a String),
    Short,
    Boolean,
    Array(Box<FieldType<'a>>),
}

impl<'a> Display for FieldType<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FieldType::Byte => write!(f, "B"),
            FieldType::Char => write!(f, "C"),
            FieldType::Double => write!(f, "D"),
            FieldType::Float => write!(f, "F"),
            FieldType::Int => write!(f, "I"),
            FieldType::Long => write!(f, "J"),
            FieldType::Reference(str) => write!(f, "L{}", str),
            FieldType::Short => write!(f, "S"),
            FieldType::Boolean => write!(f, "Z"),
            FieldType::Array(field_type) => {
                write!(f, "[")?;
                field_type.fmt(f)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ParameterDescriptor<'a> {
    FieldType(FieldType<'a>),
}

impl<'a> Display for ParameterDescriptor<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParameterDescriptor::FieldType(field_type) => field_type.fmt(f),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ReturnDescriptor<'a> {
    FieldType(FieldType<'a>),
    Void(VoidDescriptor),
}

impl<'a> Display for ReturnDescriptor<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ReturnDescriptor::FieldType(field_type) => field_type.fmt(f),
            ReturnDescriptor::Void(void) => void.fmt(f),
        }
    }
}

#[derive(Debug, Clone)]
pub enum VoidDescriptor {
    Void,
}

impl Display for VoidDescriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            VoidDescriptor::Void => write!(f, "V"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::descriptor::FieldType;

    #[test]
    fn test_fmt_field_type() {
        let class_name = &String::from("java/lang/String");
        let field_type = FieldType::Array(Box::new(FieldType::Array(Box::new(
            FieldType::Reference(class_name),
        ))));
        println!("{}", field_type);
    }
}
