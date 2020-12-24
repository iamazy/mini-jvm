use std::fmt::{self, Display, Formatter};
use std::sync::Arc;
use crate::oops::symbol::Symbol;

pub struct ClassPtr(u64);

#[derive(Debug, Clone)]
pub struct Class {
    pub access_flags: u16,
    // java/lang/String, etc
    pub name: Arc<Symbol>,
    // None for java/lang/Object
    pub super_class: Option<Arc<ClassPtr>>,
    // First subclass (None if none); sub_class.next_sibling() is next one
    pub sub_class: Option<Arc<ClassPtr>>
}

impl Class {

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