use crate::attribute::Attribute;
use crate::constant::Constant;
use crate::error::Error;
use crate::field::FieldInfo;
use crate::method::MethodInfo;
use crate::{ConstantPoolRef, TryFromCp, TryInto, MAGIC};
use bytes::{Buf, BufMut, BytesMut};
use std::convert::TryFrom;
use std::sync::Arc;

pub type ClassFileRef = Arc<ClassFile>;

#[derive(Debug, Clone)]
pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: ConstantPoolRef,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<Constant>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<Attribute>,
}

impl TryFrom<&mut BytesMut> for ClassFile {
    type Error = Error;

    fn try_from(buf: &mut BytesMut) -> Result<Self, Self::Error> {
        let magic = buf.get_u32();
        assert_eq!(magic, MAGIC);
        let minor_version = buf.get_u16();
        let major_version = buf.get_u16();
        let constant_pool_count = buf.get_u16();
        let mut constant_pool: Vec<Constant> = Vec::with_capacity(constant_pool_count as usize - 1);
        for _ in 0..constant_pool_count - 1 {
            constant_pool.push(Constant::try_from(&mut *buf)?);
        }
        let constant_pool = Arc::new(constant_pool);
        let access_flags = buf.get_u16();
        let this_class = buf.get_u16();
        let super_class = buf.get_u16();
        let interface_count = buf.get_u16();
        let mut interfaces: Vec<Constant> = vec![];
        for _ in 0..interface_count {
            let constant = Constant::try_from(&mut *buf)?;
            if let Constant::Class { .. } = constant {
                interfaces.push(constant);
            } else {
                return Err(Error::MismatchConstantType);
            }
        }
        let fields_count = buf.get_u16();
        let mut fields: Vec<FieldInfo> = vec![];
        for _ in 0..fields_count {
            fields.push(FieldInfo::try_from_cp(buf, &constant_pool)?);
        }
        let methods_count = buf.get_u16();
        let mut methods: Vec<MethodInfo> = vec![];
        for _ in 0..methods_count {
            methods.push(MethodInfo::try_from_cp(buf, &constant_pool)?);
        }
        let attributes_count = buf.get_u16();
        let mut attributes: Vec<Attribute> = vec![];
        for _ in 0..attributes_count {
            attributes.push(Attribute::try_from_cp(buf, &constant_pool)?);
        }
        Ok(ClassFile {
            magic,
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        })
    }
}

impl<T> TryInto<&mut T, usize> for ClassFile
where
    T: BufMut,
{
    type Error = Error;

    fn try_into(&self, buf: &mut T) -> Result<usize, Self::Error> {
        let mut len: usize = 0;
        buf.put_u32(self.magic);
        buf.put_u16(self.minor_version);
        buf.put_u16(self.major_version);
        buf.put_u16(self.constant_pool.len() as u16);
        len += 10;
        for constant in &*self.constant_pool {
            len += constant.to_buf(buf)?;
        }
        buf.put_u16(self.access_flags);
        buf.put_u16(self.this_class);
        buf.put_u16(self.super_class);
        buf.put_u16(self.interfaces.len() as u16);
        buf.put_u16(self.fields.len() as u16);
        len += 10;
        for field in &self.fields {
            len += field.try_into(buf)?;
        }
        buf.put_u16(self.methods.len() as u16);
        len += 2;
        for method in &self.methods {
            len += method.try_into(buf)?;
        }
        buf.put_u16(self.attributes.len() as u16);
        len += 2;
        for attribute in &self.attributes {
            len += attribute.try_into(buf)?;
        }
        Ok(len)
    }
}

#[cfg(test)]
mod test {
    use crate::class_file::ClassFile;
    use bytes::{BufMut, BytesMut};
    use std::convert::TryFrom;
    use std::io::Read;

    #[test]
    fn read_class_file() {
        let file = std::fs::File::open("tests/HelloWorld.class").unwrap();
        let bytes: Vec<u8> = file.bytes().map(|x| x.unwrap()).collect();
        let mut buf = BytesMut::with_capacity(64);
        buf.put_slice(bytes.as_slice());
        let class_file = ClassFile::try_from(&mut buf).unwrap();
        println!("{:?}", class_file);
    }
}
