use crate::oops::symbol::Symbol;
use classfile::access_flags::AccessFlags;
use std::fmt::{self, Display, Formatter};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ClassPtr(u64);

impl ClassPtr {
    pub fn new(class: Class) -> Arc<ClassPtr> {
        let class = Box::new(class);
        let ptr = Box::into_raw(class) as u64;
        Arc::new(ClassPtr(ptr))
    }
}

impl Drop for ClassPtr {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(self.0 as *mut Class) };
    }
}

#[derive(Debug, Clone)]
pub struct Class {
    pub access_flags: AccessFlags,
    // java/lang/String, etc
    pub name: Arc<Symbol>,
    // None for java/lang/Object
    pub super_class: Option<Arc<ClassPtr>>,
    // First subclass (None if none); sub_class.next_sibling() is next one
    pub sub_class: Option<Arc<ClassPtr>>,
}

impl Class {
    // pub fn get_name<'a>(&self, constant_pool: &'a Vec<Constant>) -> Option<&'a String>{
    //     if let Constant::Class { name_index } = self {
    //         return get_utf8(constant_pool, *name_index as usize)
    //     }
    //     None
    // }

    pub fn is_subclass_of(&self, class: &Class) {
        unimplemented!()
    }

    pub fn is_final(&self) -> bool {
        unimplemented!()
    }

    pub fn is_interface(&self) -> bool {
        unimplemented!()
    }

    pub fn is_abstract(&self) -> bool {
        unimplemented!()
    }

    pub fn is_super(&self) -> bool {
        unimplemented!()
    }

    pub fn is_synthetic(&self) -> bool {
        unimplemented!()
    }

    pub fn has_finalizer(&self) -> bool {
        unimplemented!()
    }

    pub fn has_final_method(&self) -> bool {
        unimplemented!()
    }

    pub fn is_cloneable(&self) -> bool {
        unimplemented!()
    }

    pub fn next_sibling(&self) -> Arc<ClassPtr> {
        unimplemented!()
    }

    pub fn append_to_sibling_list(&self) {
        unimplemented!()
    }

    pub fn super_class(&self) -> Arc<ClassPtr> {
        unimplemented!()
    }

    pub fn sub_class(&self) -> Arc<ClassPtr> {
        unimplemented!()
    }

    pub fn initialize(&self) {
        unimplemented!()
    }
}

impl Display for Class {
    // print class code
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

/// See "The Java Virtual Machine Specification" section 2.16.2-5 for a detailed description
/// of the class loading & initialization procedure, and the use of the states.
#[derive(Debug, Clone)]
pub enum ClassState {
    // allocated (but not yet linked)
    Allocated,
    // loaded and inserted in class hierarchy (but not linked yet)
    Loaded,
    // successfully linked/verified (but not initialized yet)
    Linked,
    // currently running class initializer
    BeingInitialized,
    // initialized (successful final state)
    FullyInitialized,
    // error happened during initialization
    InitializationError,
}
