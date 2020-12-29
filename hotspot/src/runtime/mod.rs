use crate::oops::Oop;
use std::fmt::{self, Display, Formatter};

pub mod thread;
pub mod frame;
pub mod local_vars;
pub mod constant_pool;

#[derive(Debug, Clone)]
pub enum Slot {
    Oop(Oop),
    Nop
}