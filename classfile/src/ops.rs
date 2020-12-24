#![allow(unused)]

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