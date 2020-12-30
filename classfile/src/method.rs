#[allow(dead_code)]
use crate::attribute::Attribute;
use bytes::{BytesMut, BufMut, Buf};
use crate::error::Error;
use crate::constant::ConstantPool;
use crate::{TryFromCp, TryInto};

#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<Attribute>,
}

impl TryFromCp<&mut BytesMut> for MethodInfo {
    type Error = Error;

    fn try_from_cp(buf: &mut BytesMut, constant_pool: &ConstantPool) -> Result<Self, Self::Error> {
        let access_flags = buf.get_u16();
        let name_index = buf.get_u16();
        let descriptor_index = buf.get_u16();
        let attribute_count = buf.get_u16();
        let mut attributes: Vec<Attribute> = vec![];
        for _ in 0..attribute_count {
            attributes.push(Attribute::try_from_cp(buf, constant_pool)?);
        }
        Ok(MethodInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        })
    }
}

impl<T> TryInto<&mut T, usize> for MethodInfo where
    T: BufMut {
    type Error = Error;

    fn try_into(&self, buf: &mut T) -> Result<usize, Self::Error> {
        let mut len: usize = 0;
        buf.put_u16(self.access_flags);
        buf.put_u16(self.name_index);
        buf.put_u16(self.descriptor_index);
        buf.put_u16(self.attributes.len() as u16);
        len += 8;
        for attribute in &self.attributes {
            len += attribute.try_into(buf)?;
        }
        Ok(len)
    }
}