use crate::oops::Oop;
use std::fmt::{self, Display, Formatter};

pub mod thread;
pub mod frame;
pub mod local_vars;

#[derive(Debug, Clone)]
pub enum Slot {
    Oop(Oop),
    Nop
}

impl Display for Slot {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}