use crate::oops::class::ClassPtr;

def_ptr!(CharArrayPtr, Vec<char>);
def_ptr!(BoolArrayPtr, Vec<bool>);
def_ptr!(ByteArrayPtr, Vec<u8>);
def_ptr!(IntArrayPtr, Vec<i32>);
def_ptr!(LongArrayPtr, Vec<i64>);
def_ptr!(ShortArrayPtr, Vec<i16>);
def_ptr!(FloatArrayPtr, Vec<f32>);
def_ptr!(DoubleArrayPtr, Vec<f64>);