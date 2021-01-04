use crate::basic_type::BasicType;
use crate::oops::Oop;
use crate::runtime::class_loader::ClassLoader;
use crate::types::{ClassRef, FieldIdRef, MethodIdRef};
use classfile::access_flags::AccessFlags;
use classfile::class_file::ClassFileRef;
use classfile::{BytesRef, ConstantPoolRef};
use parking_lot::ReentrantMutex;
use std::fmt::{self, Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Eq, PartialEq)]
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

    pub fn get_instance(&self) -> &ObjectInstance {
        let class = self.get_class();
        match &class.instance {
            Instance::Instance(instance) => instance,
            _ => unreachable!(),
        }
    }
}

pub struct Class {
    mutex: ReentrantMutex<()>,
    clint_mutex: Arc<Mutex<()>>,
    class_state: ClassState,
    pub access_flags: AccessFlags,
    pub constant_pool: ConstantPoolRef,
    // java/lang/String, etc
    pub name: BytesRef,
    // None for java/lang/Object
    pub super_class: Option<ClassRef>,
    pub sub_classes: Option<Vec<ClassRef>>,
    pub class_file: ClassFileRef,
    // None for the bootstrap loader
    pub class_loader: Option<ClassLoader>,
    pub instance: Instance,
}

impl Class {

    pub fn get_name(&self) -> BytesRef {
        self.name.clone()
    }

    pub fn is_subclass_of(&self, class: ClassRef) -> bool {
        match &self.sub_classes {
            Some(sub_classes) => sub_classes.contains(&class),
            None => false,
        }
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
        self.access_flags().is_interface()
    }

    pub fn is_abstract(&self) -> bool {
        self.access_flags().is_abstract()
    }

    pub fn is_super(&self) -> bool {
        self.access_flags().is_super()
    }

    pub fn is_synthetic(&self) -> bool {
        self.access_flags().is_synthetic()
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

    pub fn append_subclass(&mut self, class: ClassRef) {
        match &mut self.sub_classes {
            Some(ref mut sub_classes) => {
                sub_classes.push(class);
            }
            None => {}
        }
    }

    pub fn super_class(&self) -> Option<ClassRef> {
        self.super_class.clone()
    }

    pub fn sub_classes(&self) -> Option<Vec<ClassRef>>  {
        self.sub_classes.clone()
    }

    pub fn initialize(&self) {
        unimplemented!()
    }

    pub fn get_instance_type(&self) -> InstanceType {
        match &self.instance {
            Instance::Instance(_) => InstanceType::ObjectInstance,
            Instance::ObjectArray(_) => InstanceType::ObjectArrayInstance,
            Instance::TypeArray(_) => InstanceType::TypeArrayInstance
        }
    }
}

impl Display for Class {
    // print class code
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
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

pub enum Instance {
    Instance(ObjectInstance),
    ObjectArray(ObjectArrayInstance),
    TypeArray(TypeArrayInstance),
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum InstanceType {
    ObjectInstance,
    ObjectArrayInstance,
    TypeArrayInstance,
}

pub struct ObjectInstance {
    pub class: ClassRef,
    pub interfaces: Vec<ClassRef>,
    pub methods: Vec<MethodIdRef>,
    // field, value
    pub fields: Vec<(FieldIdRef, Oop)>,
    pub mirror: Option<Oop>,
}

pub struct ObjectArrayInstance {
    pub dimension: usize,
    pub element_type: BasicType,
    pub element_class: ClassRef,
    pub bottom_class: ClassRef,
    pub mirror: Option<Oop>,
}

pub struct TypeArrayInstance {
    pub dimension: usize,
    pub element_type: BasicType,
    pub max_length: usize,
    pub mirror: Option<Oop>,
}

impl ObjectInstance {
    #[inline]
    pub fn is_gc_marked(&self) -> bool {
        unimplemented!()
    }

    pub fn static_field_size(&self) -> usize {
        let mut size: usize = 0;
        for field in &self.fields {
            if !field.0.field.is_static() {
                size += 1;
            }
        }
        size
    }

    // not include static field
    pub fn field_size(&self) -> usize {
        self.fields.len() - self.static_field_size()
    }
}
