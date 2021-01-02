use crate::types::{BytesRef, MethodIdRef};
use classfile::constant::ConstantPool;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

pub struct Frame {
    pub id: usize,
    pub pc: AtomicUsize,
    pub stack_pointer: usize,
    pub constant_pool: Arc<ConstantPool>,
    pub method: MethodIdRef,
    pub code: BytesRef,
}
