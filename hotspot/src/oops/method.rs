use crate::basic_type::BasicType;
use crate::oops::class::ClassPtr;
use crate::oops::symbol::Symbol;
use classfile::access_flags::AccessFlags;
use classfile::constant::ConstantPool;
use classfile::method::MethodInfo;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub struct Method<'a> {
    pub access_flags: AccessFlags,
    // method holder
    pub class: ClassPtr,
    // offset in class method list
    pub offset: usize,
    pub method_info: &'a MethodInfo,
}

impl<'a> Method<'a> {
    pub fn class_name(&self) -> &'a String {
        unimplemented!()
    }

    pub fn return_type(&self) -> BasicType {
        unimplemented!()
    }

    // Access flags
    pub fn access_flags(&self) -> &AccessFlags {
        &self.access_flags
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

    pub fn is_package_private(&self) -> bool {
        !self.is_public() && !self.is_private() && !self.is_protected()
    }

    pub fn is_static(&self) -> bool {
        self.access_flags().is_static()
    }

    pub fn is_final(&self) -> bool {
        self.access_flags().is_final()
    }

    pub fn is_synchronized(&self) -> bool {
        self.access_flags().is_synchronized()
    }

    pub fn is_native(&self) -> bool {
        self.access_flags().is_native()
    }

    pub fn is_abstract(&self) -> bool {
        self.access_flags().is_abstract()
    }

    pub fn is_strict(&self) -> bool {
        self.access_flags().is_strict()
    }

    pub fn is_synthetic(&self) -> bool {
        self.access_flags().is_synthetic()
    }

    // interface method declared with 'default' - excludes private interface methods
    pub fn is_default_method(&self) -> bool {
        unimplemented!()
    }

    pub fn parameters_length(&self) -> usize {
        unimplemented!()
    }

    pub fn is_accessor(&self) -> bool {
        unimplemented!()
    }

    pub fn is_getter(&self) -> bool {
        unimplemented!()
    }

    pub fn is_setter(&self) -> bool {
        unimplemented!()
    }

    pub fn is_constant_getter(&self) -> bool {
        unimplemented!()
    }

    pub fn is_initializer(&self) -> bool {
        unimplemented!()
    }

    pub fn is_static_initializer(&self) -> bool {
        unimplemented!()
    }

    pub fn is_object_initializer(&self) -> bool {
        unimplemented!()
    }

    pub fn print_method_descriptor(&self) {
        unimplemented!()
    }

    // name
    pub fn name(&self) -> &'a Symbol {
        unimplemented!()
    }

    pub fn name_index(&self) -> u16 {
        self.method_info.name_index
    }

    pub fn signature(&self) -> &'a Symbol {
        unimplemented!()
    }

    pub fn signature_index(&self) -> u16 {
        unimplemented!()
    }

    // generics support
    pub fn generic_signature(&self) -> &'a Symbol {
        unimplemented!()
    }

    pub fn generic_signature_index(&self) -> u16 {
        unimplemented!()
    }

    pub fn annotations(&self) -> Option<Vec<u8>> {
        unimplemented!()
    }

    pub fn parameter_annotations(&self) -> Option<Vec<u8>> {
        unimplemented!()
    }

    pub fn type_annotations(&self) -> Option<Vec<u8>> {
        unimplemented!()
    }

    pub fn annotation_default(&self) -> Option<Vec<u8>> {
        unimplemented!()
    }

    pub fn max_locals(&self) -> usize {
        return match self.method_info.get_code() {
            Some(code) => code.max_locals as usize,
            None => 0,
        };
    }

    pub fn max_stack(&self) -> usize {
        return match self.method_info.get_code() {
            Some(code) => code.max_stack as usize,
            None => 0,
        };
    }
}

impl<'a> Display for Method<'a> {
    // print method code
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}
