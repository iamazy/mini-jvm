#[derive(Debug, Clone)]
pub struct AccessFlags(u16);

impl AccessFlags {
    pub fn is_public(&self) -> bool {
        self.0 & AccessFlag::ACC_PUBLIC.bits != 0
    }

    pub fn is_private(&self) -> bool {
        self.0 & AccessFlag::ACC_PRIVATE.bits != 0
    }

    pub fn is_protected(&self) -> bool {
        self.0 & AccessFlag::ACC_PROTECTED.bits != 0
    }

    pub fn is_static(&self) -> bool {
        self.0 & AccessFlag::ACC_STATIC.bits != 0
    }

    pub fn is_final(&self) -> bool {
        self.0 & AccessFlag::ACC_FINAL.bits != 0
    }

    pub fn is_synchronized(&self) -> bool {
        self.0 & AccessFlag::ACC_SYNCHRONIZED.bits != 0
    }

    pub fn is_super(&self) -> bool {
        self.0 & AccessFlag::ACC_SUPER.bits != 0
    }

    pub fn is_volatile(&self) -> bool {
        self.0 & AccessFlag::ACC_VOLATILE.bits != 0
    }

    pub fn is_transient(&self) -> bool {
        self.0 & AccessFlag::ACC_TRANSIENT.bits != 0
    }

    pub fn is_native(&self) -> bool {
        self.0 & AccessFlag::ACC_NATIVE.bits != 0
    }

    pub fn is_interface(&self) -> bool {
        self.0 & AccessFlag::ACC_INTERFACE.bits != 0
    }

    pub fn is_abstract(&self) -> bool {
        self.0 & AccessFlag::ACC_ABSTRACT.bits != 0
    }

    pub fn is_strict(&self) -> bool {
        self.0 & AccessFlag::ACC_STRICT.bits != 0
    }

    // Attribute flags
    pub fn is_synthetic(&self) -> bool {
        self.0 & AccessFlag::ACC_SYNTHETIC.bits != 0
    }
}

bitflags! {
    struct AccessFlag: u16 {
        const ACC_PUBLIC = 0x0001;
        const ACC_PRIVATE = 0x0002;
        const ACC_PROTECTED = 0x0004;
        const ACC_STATIC = 0x0008;
        const ACC_FINAL = 0x0010;
        const ACC_SUPER = 0x0020;
        const ACC_SYNCHRONIZED = 0x0020;
        const ACC_BRIDGE = 0x0040;
        const ACC_VOLATILE = 0x0040;
        const ACC_VARARGS = 0x0080;
        const ACC_TRANSIENT = 0x0080;
        const ACC_NATIVE = 0x0100;
        const ACC_INTERFACE = 0x0200;
        const ACC_ABSTRACT = 0x0400;
        const ACC_STRICT = 0x0800;
        const ACC_SYNTHETIC = 0x1000;
        const ACC_ANNOTATION = 0x2000;
        const ACC_ENUM = 0x4000;
    }
}
