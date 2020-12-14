#![allow(unused)]

pub const MAGIC: [u8; 4] = [0xCA, 0xFE, 0xBA, 0xBE];

/// Class access and property modifiers
// class
pub const ACC_PUBLIC: u16 = 0x0001;
// class
pub const ACC_PRIVATE: u16 = 0x0002;
// class
pub const ACC_PROTECTED: u16 = 0x0004;
// class
pub const ACC_STATIC: u16 = 0x0008;
// class
pub const ACC_FINAL: u16 = 0x0010;
// class
pub const ACC_SUPER: u16 = 0x0020;
// class
pub const ACC_SYNCHRONIZED: u16 = 0x0020;

pub const ACC_BRIDGE: u16 = 0x0040;

pub const ACC_VOLATILE: u16 = 0x0040;

pub const ACC_VARARGS: u16 = 0x0080;

pub const ACC_TRANSIENT: u16 = 0x0080;
// method
pub const ACC_NATIVE: u16 = 0x0100;


// class
pub const ACC_INTERFACE: u16 = 0x0200;
// class
pub const ACC_ABSTRACT: u16 = 0x0400;

pub const ACC_STRICT: u16 = 0x0800;
// class
pub const ACC_SYNTHETIC: u16 = 0x1000;
// class
pub const ACC_ANNOTATION: u16 = 0x2000;
// class
pub const ACC_ENUM: u16 = 0x4000;



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