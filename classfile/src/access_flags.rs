
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