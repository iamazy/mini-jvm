use crate::types::{BytesRef, ClassRef, FieldIdRef, MethodIdRef};
use classfile::access_flags::AccessFlags;
use classfile::constant::ConstantPool;
use parking_lot::ReentrantMutex;
use std::fmt::{self, Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct ClassPtr(u64);

impl ClassPtr {
    pub fn new(class: &Class) -> ClassRef {
        let class = Box::new(class);
        let ptr = Box::into_raw(class) as u64;
        Arc::new(ClassPtr(ptr))
    }
}

impl Drop for ClassPtr {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.0 as *mut Class);
        }
    }
}

impl ClassPtr {
    fn raw_ptr(&self) -> *const Class {
        self.0 as *const Class
    }

    fn mut_raw_ptr(&self) -> *mut Class {
        self.0 as *mut Class
    }
}

impl ClassPtr {
    pub fn name(&self) -> &BytesRef {
        let ptr = self.raw_ptr();
        unsafe { &(*ptr).name }
    }

    pub fn get_class(&self) -> &Class {
        let ptr = self.raw_ptr();
        unsafe { &(*ptr) }
    }

    pub fn get_mut_class(&self) -> &mut Class {
        let ptr = self.mut_raw_ptr();
        unsafe { &mut (*ptr) }
    }
}

#[derive(Debug)]
pub struct Class {
    mutex: ReentrantMutex<()>,
    clint_mutex: Arc<Mutex<()>>,
    class_state: ClassState,
    pub access_flags: AccessFlags,
    pub constant_pool: Arc<Box<ConstantPool>>,
    // java/lang/String, etc
    pub name: BytesRef,
    // None for java/lang/Object
    pub super_class: Option<ClassRef>,
    // First subclass (None if none); sub_class.next_sibling() is next one
    pub sub_class: Option<ClassRef>,
}

impl Class {
    // pub fn get_name<'a>(&self, constant_pool: &'a Vec<Constant>) -> Option<&'a String>{
    //     if let Constant::Class { name_index } = self {
    //         return get_utf8(constant_pool, *name_index as usize)
    //     }
    //     None
    // }

    pub fn is_subclass_of(&self, _class: ClassRef) -> bool {
        unimplemented!()
    }

    pub fn access_flags(&self) -> &AccessFlags {
        &self.access_flags
    }

    pub fn is_public(&self) -> bool {
        self.access_flags().is_public()
    }

    pub fn is_final(&self) -> bool {
        self.access_flags().is_final()
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

    pub fn next_sibling(&self) -> ClassRef {
        unimplemented!()
    }

    pub fn append_to_sibling_list(&self) {
        unimplemented!()
    }

    pub fn super_class(&self) -> ClassRef {
        unimplemented!()
    }

    pub fn sub_class(&self) -> ClassRef {
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

#[derive(Debug, Clone)]
pub enum ClassType {
    Instance,
    Array,
    ObjectArray,
    TypeArray,
}

pub struct Instance {
    pub class: ClassPtr,
    pub methods: Vec<MethodIdRef>,
    pub fields: Vec<FieldIdRef>,
}
