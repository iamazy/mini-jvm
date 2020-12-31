use crate::oops::Oop;
use std::fmt::{self, Display, Formatter};

pub mod constant_pool;
pub mod frame;
pub mod local_vars;
pub mod thread;

#[derive(Debug, Clone)]
pub enum Slot {
    Oop(Oop),
    Nop,
}
