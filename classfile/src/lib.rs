#[allow(unused)]
use bytes::{BufMut, BytesMut, Buf, Bytes};
use crate::error::Error;

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


pub const MAGIC: [u8; 4] = [0xCA, 0xFE, 0xBA, 0xBE];

/// Tag values for the constant pool entries
pub const CONSTANT_CLASS_TAG: u8 = 7;
pub const CONSTANT_FIELD_REF_TAG: u8 = 9;
pub const CONSTANT_METHOD_REF_TAG: u8 = 10;
pub const CONSTANT_INTERFACE_METHOD_REF_TAG: u8 = 11;
pub const CONSTANT_STRING_TAG: u8 = 8;
pub const CONSTANT_INTEGER_TAG: u8 = 3;
pub const CONSTANT_FLOAT_TAG: u8 = 4;
pub const CONSTANT_LONG_TAG: u8 = 5;
pub const CONSTANT_DOUBLE_TAG: u8 = 6;
pub const CONSTANT_NAME_AND_TYPE_TAG: u8 = 12;
pub const CONSTANT_UTF8_TAG: u8 = 1;
pub const CONSTANT_METHOD_HANDLE_TAG: u8 = 15;
pub const CONSTANT_METHOD_TYPE_TAG: u8 = 16;
pub const CONSTANT_DYNAMIC_TAG: u8 = 17;
pub const CONSTANT_INVOKE_DYNAMIC_TAG: u8 = 18;
pub const CONSTANT_MODULE_TAG: u8 = 19;
pub const CONSTANT_PACKAGE_TAG: u8 = 20;


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

pub trait FromToBytes<R> {
    fn to_buf(&self, buf: &mut impl BufMut) -> Result<usize, Error>;
    fn from_buf(buf: &mut BytesMut) -> Result<R, Error>;
    fn length(&self) -> usize;
}