use crate::constant::Constant;
use crate::field::Field;

#[derive(Debug)]
pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: Vec<Constant>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<Constant>,
    pub fields_count: u16,
    pub fields: Vec<Field>

}