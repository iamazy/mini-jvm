use bytes::{BufMut, BytesMut};
use crate::error::Error;

pub mod class_file;
pub mod constant;
pub mod field;
pub mod attribute;
pub mod method;
pub mod ops;
pub mod class_reader;
pub mod error;

pub trait Frame<R> {
    fn to_buf(&self, buf: &mut impl BufMut) -> usize;
    fn from_buf(buf: &mut BytesMut) -> Result<R, Error>;
    fn length(&self) -> usize;
}