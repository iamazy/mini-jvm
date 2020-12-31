use classfile::constant::ConstantPool;

pub struct ConstantPoolCache<'a> {
    constant_pool: &'a ConstantPool,
}
