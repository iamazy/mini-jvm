/// Class access and property modifiers
#[derive(Debug, Clone)]
pub enum AccessFlag {
    Public,
    Private,
    Protected,
    Static,
    Final,
    Super,
    Synchronized,
    Bridge,
    Volatile,
    Varargs,
    Transient,
    Native,
    Interface,
    Abstract,
    Strict,
    Synthetic,
    Annotation,
    Enum
}

impl Into<u16> for AccessFlag {
    fn into(self) -> u16 {
        match self {
            AccessFlag::Public => 0x0001u16,
            AccessFlag::Private => 0x0002u16,
            AccessFlag::Protected => 0x0004u16,
            AccessFlag::Static => 0x0008u16,
            AccessFlag::Final => 0x0010u16,
            AccessFlag::Super => 0x0020u16,
            AccessFlag::Synchronized => 0x0020u16,
            AccessFlag::Bridge => 0x0040u16,
            AccessFlag::Volatile => 0x0040u16,
            AccessFlag::Varargs => 0x0080u16,
            AccessFlag::Transient => 0x0080u16,
            AccessFlag::Native => 0x0100u16,
            AccessFlag::Interface => 0x0200u16,
            AccessFlag::Abstract => 0x0400u16,
            AccessFlag::Strict => 0x0800u16,
            AccessFlag::Synthetic => 0x1000u16,
            AccessFlag::Annotation => 0x2000u16,
            AccessFlag::Enum => 0x4000u16
        }
    }
}