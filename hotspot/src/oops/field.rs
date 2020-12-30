use crate::oops::symbol::Symbol;
use classfile::access_flags::AccessFlags;
use classfile::field::FieldInfo;
use crate::oops::class::ClassPtr;
use classfile::descriptor::FieldType;

#[derive(Debug, Clone)]
pub struct Field<'a> {
    pub access_flags: AccessFlags,
    pub class: ClassPtr,
    pub offset: usize,
    pub field_type: FieldType<'a>,
    pub field_info: &'a FieldInfo,
}

impl<'a> Field<'a> {

    pub fn class_name(&self) -> &'a String {
        unimplemented!()
    }

    // Access flags
    pub fn access_flags(&self) -> &'a AccessFlags {
        &self.access_flags
    }

    pub fn name(&self) -> &'a Symbol{
        unimplemented!()
    }

    pub fn name_index(&self) -> u16 {
        self.field_info.name_index
    }

    pub fn signature(&self) -> &'a Symbol {
        unimplemented!()
    }

    pub fn signature_index(&self) -> u16 {
        unimplemented!()
    }

}