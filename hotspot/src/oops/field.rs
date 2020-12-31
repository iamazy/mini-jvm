use crate::oops::class::ClassPtr;
use crate::oops::symbol::Symbol;
use classfile::access_flags::AccessFlags;
use classfile::descriptor::FieldType;
use classfile::field::FieldInfo;

#[derive(Debug, Clone)]
pub struct Field<'a> {
    pub access_flags: AccessFlags,
    pub class: ClassPtr,
    pub offset: usize,
    pub field_type: FieldType<'a>,
    pub field_info: &'a FieldInfo,
}

impl<'a> Field<'a> {
    pub fn class_name(&self) -> &String {
        unimplemented!()
    }

    // Access flags
    pub fn access_flags(&self) -> &AccessFlags {
        &self.access_flags
    }

    pub fn name(&self) -> &Symbol {
        unimplemented!()
    }

    pub fn name_index(&self) -> u16 {
        self.field_info.name_index
    }

    pub fn signature(&self) -> &Symbol {
        unimplemented!()
    }

    pub fn signature_index(&self) -> u16 {
        unimplemented!()
    }
}
