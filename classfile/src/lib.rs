#[allow(unused)]
#[macro_use]
extern crate bitflags;

use std::sync::Arc;

use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::constant::Constant;
use crate::error::Error;

pub mod access_flags;
pub mod attribute;
pub mod class_file;
pub mod class_reader;
pub mod constant;
pub mod error;
pub mod field;
pub mod method;

pub const MAGIC: u32 = 0xCAFEBABE;

pub type BytesRef = Arc<Vec<u8>>;
pub type ConstantPoolRef = Arc<Vec<Constant>>;

pub fn write_string(string: String, buf: &mut impl BufMut) -> usize {
    write_bytes(Bytes::from(string), buf)
}

pub fn read_string(buf: &mut BytesMut) -> Result<String, Error> {
    String::from_utf8(read_bytes(buf)?.to_vec())
        .map_err(|e| Error::InvalidString(e.utf8_error().to_string()))
}

pub fn write_utf8(bytes: Vec<u8>, buf: &mut impl BufMut) -> usize {
    write_bytes(Bytes::from(bytes), buf)
}

pub fn read_utf8(buf: &mut BytesMut) -> Result<Vec<u8>, Error> {
    Ok(read_bytes(buf)?.to_vec())
}

pub fn write_bytes(bytes: Bytes, buf: &mut impl BufMut) -> usize {
    let len = bytes.len();
    buf.put_u16(len as u16);
    buf.put_slice(bytes.bytes());
    len + 2
}

pub fn read_bytes(buf: &mut BytesMut) -> Result<Bytes, Error> {
    let len = buf.get_u16() as usize;
    if len > buf.remaining() {
        Err(Error::InvalidLength)
    } else {
        Ok(buf.split_to(len).split().freeze())
    }
}

pub trait TryInto<T, S>: Sized {
    type Error;
    fn try_into(&self, t: T) -> Result<S, Self::Error>;
}

pub trait TryFromCp<T>: Sized {
    type Error;
    fn try_from_cp(value: T, constant_pool: &ConstantPoolRef) -> Result<Self, Self::Error>;
}
