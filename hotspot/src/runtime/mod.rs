use crate::oops::Oop;
use crate::runtime::local_vars::LocalVars;
use crate::stack::Stack;
use std::cell::RefCell;

pub mod class_loader;
pub mod constant_pool;
pub mod frame;
pub mod local_vars;
pub mod park;
pub mod thread;

#[derive(Debug, Clone)]
pub enum Slot {
    Oop(Oop),
    Nop,
}

pub struct DataArea {
    pub local: RefCell<LocalVars>,
    pub stack: RefCell<Stack<Slot>>,
    pub value: RefCell<Option<Oop>>,
}

unsafe impl Sync for DataArea {}

impl DataArea {
    pub fn new(max_locals: usize, max_stack: usize) -> Self {
        let local = RefCell::new(LocalVars::new(max_locals));
        let stack = RefCell::new(Stack::new(max_stack));

        Self {
            local,
            stack,
            value: RefCell::new(None),
        }
    }
}
