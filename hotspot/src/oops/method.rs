use std::sync::Arc;
use classfile::attribute::{Exception, LineNumber, CodeAttribute};
use crate::basic_type::BasicType;
use std::fmt::{self, Display, Formatter};
use crate::oops::class::ClassPtr;
use crate::oops::symbol::Symbol;
use classfile::method::MethodInfo;
use classfile::access_flags::AccessFlag;

#[derive(Debug, Clone)]
pub struct Method<'a> {
    pub name: Arc<Symbol>,
    pub signature: &'a String,
    // method holder
    pub class: Arc<ClassPtr>,
    pub const_method: &'a MethodInfo
}

impl<'a> Method<'a> {

    pub fn class_name(&self) -> &'a String {
        unimplemented!()
    }

    pub fn return_type(&self) -> BasicType {
        unimplemented!()
    }

    // Access flags
    pub fn is_public(&self) -> bool {
        unimplemented!()
    }

    pub fn is_private(&self) -> bool {
        unimplemented!()
    }

    pub fn is_protected(&self) -> bool {
        unimplemented!()
    }

    pub fn is_package_private(&self) -> bool {
        unimplemented!()
    }

    pub fn is_static(&self) -> bool {
        unimplemented!()
    }

    pub fn is_final(&self) -> bool {
        unimplemented!()
    }

    pub fn is_synchronized(&self) -> bool {
        unimplemented!()
    }

    pub fn is_native(&self) -> bool {
        unimplemented!()
    }

    pub fn is_abstract(&self) -> bool {
        unimplemented!()
    }

    pub fn is_strict(&self) -> bool {
        unimplemented!()
    }

    pub fn is_synthetic(&self) -> bool {
        unimplemented!()
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
    pub fn name(&self) -> &'a Symbol{
        unimplemented!()
    }

    pub fn name_index(&self) -> u16 {
        unimplemented!()
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

    pub fn annotations(&self)



}

impl<'a> Display for Method<'a> {
    // print method code
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}