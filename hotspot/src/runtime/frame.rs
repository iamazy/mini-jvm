use crate::oops;
use crate::oops::Oop;
use crate::runtime::{DataArea, Slot};
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
    pub data_area: DataArea,
}

impl Frame {
    #[inline]
    pub fn push_int(&mut self, v: i32) {
        self.data_area
            .stack
            .borrow_mut()
            .push(Slot::Oop(oops::Oop::Int(v)));
    }

    #[inline]
    pub fn push_float(&mut self, v: f32) {
        self.data_area
            .stack
            .borrow_mut()
            .push(Slot::Oop(oops::Oop::Float(v)));
    }

    #[inline]
    pub fn push_double(&mut self, v: f64) {
        self.data_area
            .stack
            .borrow_mut()
            .push(Slot::Oop(oops::Oop::Double(v)));
    }

    #[inline]
    pub fn push_long(&mut self, v: i64) {
        self.data_area
            .stack
            .borrow_mut()
            .push(Slot::Oop(oops::Oop::Long(v)));
    }

    #[inline]
    pub fn push_null(&mut self) {
        self.data_area
            .stack
            .borrow_mut()
            .push(Slot::Oop(oops::Oop::Null));
    }

    #[inline]
    pub fn push_ref(&mut self, v: oops::Oop) {
        if let oops::Oop::Reference(..) = &v {
            self.data_area.stack.borrow_mut().push(Slot::Oop(v));
        } else {
            panic!("Incorrect Oop")
        }
    }

    #[inline]
    pub fn pop(&mut self) -> Oop {
        let mut stack = self.data_area.stack.borrow_mut();
        assert!(stack.size() > 0);
        match stack.pop().unwrap() {
            Slot::Oop(oop) => oop,
            _ => unreachable!(),
        }
    }
}
