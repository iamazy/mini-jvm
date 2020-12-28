use crate::oops::Oop;

pub mod thread;
pub mod frame;
pub mod local_vars;

#[derive(Debug, Clone)]
pub enum Slot {
    Oop(Oop),
    Nop
}