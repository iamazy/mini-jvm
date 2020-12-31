use std::fmt::{self, Display, Formatter};
use std::sync::Arc;

pub mod array;
pub mod class;
pub mod field;
pub mod method;
pub mod symbol;

// Ordinary Object Pointer
#[derive(Debug, Clone)]
pub enum Oop {
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Null,
    Reference(Arc<OopRef>),
}

impl Display for Oop {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Oop::Int(int) => write!(f, "Oop::Int({})", int),
            Oop::Long(long) => write!(f, "Oop::Long({})", long),
            Oop::Float(float) => write!(f, "Oop::Float({})", float),
            Oop::Double(double) => write!(f, "Oop::Double({})", double),
            Oop::Null => write!(f, "Oop::Null"),
            Oop::Reference(reference) => write!(f, "Oop::Reference({})", reference.0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OopRef(u64);

#[derive(Debug, Clone)]
pub struct InstanceOop {}

#[derive(Debug, Clone)]
pub struct MirrorOop {}
