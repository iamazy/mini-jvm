use crate::types::MethodIdRef;
use classfile::{BytesRef, ConstantPoolRef};
use std::sync::atomic::AtomicUsize;

pub struct Frame {
    pub id: usize,
    pub pc: AtomicUsize,
    pub stack_pointer: usize,
    pub constant_pool: ConstantPoolRef,
    pub method: MethodIdRef,
    pub code: BytesRef,
}
