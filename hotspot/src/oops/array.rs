use crate::oops::class::ClassPtr;
use std::sync::Arc;
use crate::oops::Oop;
use crate::types::{CharArrayPtr, BoolArrayPtr, ByteArrayPtr, IntArrayPtr, LongArrayPtr, ShortArrayPtr, FloatArrayPtr, DoubleArrayPtr};

/// The layout of array Oops is:
///
///  markWord
///  Klass*    // 32 bits if compressed but declared 64 in LP64.
///  length    // shares klass memory or allocated after declared fields.
#[derive(Debug, Clone)]
pub struct ArrayOop {
    pub class: Arc<ClassPtr>,
    pub elements: Vec<Oop>
}

#[derive(Debug, Clone)]
pub enum TypeArrayOop {
    Char(CharArrayPtr),
    Boolean(BoolArrayPtr),
    Byte(ByteArrayPtr),
    Int(IntArrayPtr),
    Long(LongArrayPtr),
    Short(ShortArrayPtr),
    Float(FloatArrayPtr),
    Double(DoubleArrayPtr)
}

#[derive(Debug, Clone)]
pub struct ObjArrayOop {

}


