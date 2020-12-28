#[allow(unused)]
use bytes::{BufMut, BytesMut, Buf, Bytes};
use crate::error::Error;
use crate::constant::Constant;

pub mod class_file;
pub mod constant;
pub mod field;
pub mod attribute;
pub mod method;
pub mod class_reader;
pub mod error;
pub mod access_flags;
pub mod descriptor;
pub mod package;
pub mod module;

pub const MAGIC: u32 = 0xCAFEBABE;

pub fn write_string(string: String, buf: &mut impl BufMut) -> usize {
    write_bytes(Bytes::from(string), buf)
}

pub fn read_string(buf: &mut BytesMut) -> Result<String, Error> {
    String::from_utf8(read_bytes(buf)?.to_vec())
        .map_err(|e| Error::InvalidString(e.utf8_error().to_string()))
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

pub trait TryFromCp<T>: Sized {
    type Error;
    fn try_from_cp(value: T, constant_pool: &Vec<Constant>) -> Result<Self, Self::Error>;
}

pub trait FromToBytes<R> {
    fn to_buf(&self, buf: &mut impl BufMut) -> Result<usize, Error>;
    fn from_buf(buf: &mut BytesMut) -> Result<R, Error>;
    fn length(&self) -> usize;
}