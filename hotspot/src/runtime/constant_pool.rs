use crate::types::{FieldIdRef, MethodIdRef};
use classfile::ConstantPoolRef;
use hashbrown::HashMap;
use std::cell::RefCell;

enum ConstantCacheType {
    Field(FieldIdRef),
    Method(MethodIdRef),
}

impl ConstantCacheType {
    fn get_field(&self) -> FieldIdRef {
        match self {
            ConstantCacheType::Field(fir) => fir.clone(),
            _ => unreachable!(),
        }
    }

    fn get_method(&self) -> MethodIdRef {
        match self {
            ConstantCacheType::Method(mir) => mir.clone(),
            _ => unreachable!(),
        }
    }
}

pub struct ConstantPoolCache {
    constant_pool: ConstantPoolRef,
    cache: RefCell<HashMap<usize, ConstantCacheType>>,
}

impl ConstantPoolCache {
    pub fn new(constant_pool: ConstantPoolRef) -> Self {
        Self {
            constant_pool,
            cache: RefCell::new(HashMap::default()),
        }
    }

    pub fn get_field(&self, _idx: usize) {}
}
