use std::os::raw::c_uchar;
use std::sync::atomic::AtomicU8;

#[derive(Debug, Clone)]
pub struct Frame {
    pub id: usize,
    pub pc: usize,
    pub stack_pointer: usize,

}