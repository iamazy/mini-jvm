use crate::runtime::Slot;
use crate::oops::{Oop, OopRef};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct LocalVars {
    slots: Vec<Slot>
}

impl LocalVars {

    pub fn new(size: usize) -> Self {
        let slots = vec![Slot::Nop; size];
        LocalVars {
            slots
        }
    }

    pub fn set_int(&mut self, pos: usize, value: i32) {
        self.slots[pos - 1] = Slot::Oop(Oop::Int(value));
    }

    pub fn get_int(&self, pos: usize) -> i32 {
        match self.slots.get(pos - 1).unwrap() {
            Slot::Oop(Oop::Int(value)) => *value,
            slot => panic!("Invalid slot: {:?}", slot)
        }
    }

    pub fn set_long(&mut self, pos: usize, value: i64) {
        self.slots[pos - 1] = Slot::Oop(Oop::Long(value));
    }

    pub fn get_long(&self, pos: usize) -> i64 {
        match self.slots.get(pos - 1).unwrap() {
            Slot::Oop(Oop::Long(value)) => *value,
            slot => panic!("Invalid slot: {:?}", slot)
        }
    }

    pub fn set_float(&mut self, pos: usize, value: f32) {
        self.slots[pos - 1] = Slot::Oop(Oop::Float(value));
    }

    pub fn get_float(&mut self, pos: usize) -> f32 {
        match self.slots.get(pos - 1).unwrap() {
            Slot::Oop(Oop::Float(value)) => *value,
            slot => panic!("Invalid slot: {:?}", slot)
        }
    }

    pub fn set_double(&mut self, pos: usize, value: f64) {
        self.slots[pos - 1] = Slot::Oop(Oop::Double(value));
    }

    pub fn get_double(&self, pos: usize) -> f64 {
        match self.slots.get(pos - 1).unwrap() {
            Slot::Oop(Oop::Double(value)) => *value,
            slot => panic!("Invalid slot: {:?}", slot)
        }
    }

    pub fn set_ref(&mut self, pos: usize, value: Arc<OopRef>) {
        self.slots[pos - 1] = Slot::Oop(Oop::Reference(value));
    }

    pub fn get_ref(&self, pos: usize) -> Arc<OopRef> {
        match self.slots.get(pos - 1).unwrap() {
            Slot::Oop(Oop::Reference(value)) => value.clone(),
            slot => panic!("Invalid slot: {:?}", slot)
        }
    }
}