use std::sync::Arc;

pub mod class;
pub mod method;
pub mod symbol;
pub mod array;

// Ordinary Object Pointer
#[derive(Debug, Clone)]
pub enum Oop {
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Null,
    Reference(Arc<OopRef>)
}

#[derive(Debug, Clone, Copy)]
pub struct OopRef(u64);

#[derive(Debug, Clone)]
pub struct InstanceOop {

}

#[derive(Debug, Clone)]
pub struct MirrorOop {

}