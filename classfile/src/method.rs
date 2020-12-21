use crate::attribute::Attribute;
use bytes::{BytesMut, BufMut, Buf};
use crate::error::Error;
use crate::constant::Constant;

#[derive(Debug, Clone)]
pub struct Method {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<Attribute>
}

impl Method {
    pub fn to_buf(&self, buf: &mut impl BufMut) -> Result<usize, Error> {
        let mut len: usize = 0;
        buf.put_u16(self.access_flags);
        buf.put_u16(self.name_index);
        buf.put_u16(self.descriptor_index);
        buf.put_u16(self.attributes.len() as u16);
        len += 8;
        for attribute in &self.attributes {
            len += attribute.to_buf(buf)?;
        }
        Ok(len)
    }

    pub fn from_buf(buf: &mut BytesMut, constant_pool: &Vec<Constant>) -> Result<Method, Error> {
        let access_flags = buf.get_u16();
        let name_index = buf.get_u16();
        let descriptor_index = buf.get_u16();
        let attribute_count = buf.get_u16();
        let mut attributes: Vec<Attribute> = vec![];
        for _ in 0..attribute_count {
            attributes.push(Attribute::from_buf(buf, constant_pool)?);
        }
        Ok(Method {
            access_flags,
            name_index,
            descriptor_index,
            attributes
        })
    }

    fn length(&self) -> usize {
        unimplemented!()
    }
}