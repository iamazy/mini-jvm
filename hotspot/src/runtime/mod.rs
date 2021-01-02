use crate::oops::Oop;

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
