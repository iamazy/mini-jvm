use crate::attribute::Attribute;

#[derive(Debug, Clone)]
pub struct Field {
    /// access flags
    ///
    /// ACC_PUBLIC -> 0x0001
    /// ACC_PRIVATE -> 0x002
    /// ACC_PROTECTED -> 0x0004
    /// ACC_STATIC -> 0x0008
    /// ACC_FINAL -> 0x0010
    /// ACC_VOLATILE -> 0x0040
    /// ACC_TRANSIENT -> 0x0080
    /// ACC_SYNTHETIC -> 0x1000
    /// ACC_ENUM -> 0x4000
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<Attribute>
}