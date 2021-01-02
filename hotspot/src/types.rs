use crate::oops::class::ClassPtr;
use crate::oops::field::FieldId;
use crate::oops::method::MethodId;
use std::sync::Arc;

pub type FieldIdRef = Arc<FieldId>;
pub type MethodIdRef = Arc<MethodId>;
pub type ClassRef = Arc<ClassPtr>;

def_ptr!(CharArrayPtr, Vec<char>);
def_ptr!(BoolArrayPtr, Vec<bool>);
def_ptr!(ByteArrayPtr, Vec<u8>);
def_ptr!(IntArrayPtr, Vec<i32>);
def_ptr!(LongArrayPtr, Vec<i64>);
def_ptr!(ShortArrayPtr, Vec<i16>);
def_ptr!(FloatArrayPtr, Vec<f32>);
def_ptr!(DoubleArrayPtr, Vec<f64>);

def_ref!(BytesRef, Vec<u8>);
