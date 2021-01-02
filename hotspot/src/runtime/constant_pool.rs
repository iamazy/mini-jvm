use crate::types::{FieldIdRef, MethodIdRef};
use classfile::ConstantPoolRef;
use hashbrown::HashMap;
use std::cell::RefCell;

enum ConstantCacheType {
    Field(FieldIdRef),
    Method(MethodIdRef),
}

pub struct ConstantPoolCache<'a> {
    constant_pool: &'a ConstantPoolRef,
    cache: RefCell<HashMap<usize, ConstantCacheType>>,
}

impl<'a> ConstantPoolCache<'a> {
    pub fn new(constant_pool: &'a ConstantPoolRef) -> Self {
        Self {
            constant_pool,
            cache: RefCell::new(HashMap::default()),
        }
    }

    pub fn get_field(&self, idx: usize) {}
}
