#[derive(Debug, Clone)]
pub enum BasicType {
    Boolean = 4,
    Char = 5,
    Float = 6,
    Double = 7,
    Byte = 8,
    Short = 9,
    Int = 10,
    Long = 11,
    Object = 12,
    Array = 13,
    Void = 14,
    Address = 15,
    NarrowOop = 16,
    Metadata = 17,
    NarrowClass = 18,
    // for stack value type with conflicting contents
    Conflict = 19,
    Illegal = 99,
}










