use classfile::constant::Constant;

pub struct ConstantPoolCache<'a> {
    constant_pool: &'a Vec<Constant>
}