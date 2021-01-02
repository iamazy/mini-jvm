use crate::basic_type::BasicType;
use crate::oops::Oop;
use crate::types::{BytesRef, ClassRef};
use classfile::access_flags::AccessFlags;
use classfile::field::FieldInfo;

pub struct FieldId {
    pub index: usize,
    pub field: Field,
}

pub struct Field {
    pub access_flags: AccessFlags,
    pub class: ClassRef,
    pub name: BytesRef,
    pub field_type: BasicType,
    pub field_info: FieldInfo,
    /// 1. If the `ACC_STATIC` flag in the access_flags item of the field_info structure is set,
    /// then the field represented by the field_info structure is assigned the value represented by
    /// its ConstantValue attribute as part of the initialization of the class or interface
    /// declaring the field. This occurs prior to the invocation of the class or interface initialization
    /// method of that class or interface
    /// 2. Otherwise, the Java Virtual Machine must silently ignore the attribute.
    /// 3. There may be at most one ConstantValue attribute in the attributes table of a field_info structure.
    pub constant_value: Option<Oop>,
}

impl Field {
    pub fn class_name(&self) -> &BytesRef {
        self.class.name()
    }

    pub fn field_holder(&self) -> ClassRef {
        self.class.clone()
    }

    pub fn field_info(&self) -> &FieldInfo {
        &self.field_info
    }

    // Access flags
    pub fn access_flags(&self) -> &AccessFlags {
        &self.access_flags
    }

    pub fn name(&self) -> &BytesRef {
        &self.name
    }

    pub fn name_index(&self) -> u16 {
        self.field_info.name_index
    }

    pub fn signature(&self) -> BytesRef {
        unimplemented!()
    }

    pub fn signature_index(&self) -> u16 {
        unimplemented!()
    }

    pub fn is_public(&self) -> bool {
        self.access_flags().is_public()
    }

    pub fn is_private(&self) -> bool {
        self.access_flags().is_private()
    }

    pub fn is_protected(&self) -> bool {
        self.access_flags().is_protected()
    }

    pub fn is_volatile(&self) -> bool {
        self.access_flags().is_volatile()
    }

    pub fn is_static(&self) -> bool {
        self.access_flags().is_static()
    }

    pub fn is_final(&self) -> bool {
        self.access_flags().is_final()
    }
}
