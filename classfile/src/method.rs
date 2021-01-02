#[allow(dead_code)]
use crate::attribute::Attribute;
use crate::attribute::{AttributeType, CodeAttribute};
use crate::error::Error;
use crate::{ConstantPoolRef, TryFromCp, TryInto};
use bytes::{Buf, BufMut, BytesMut};

#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<Attribute>,
    pub code_attr_index: Option<usize>,
}

impl MethodInfo {
    pub fn get_code(&self) -> Option<&CodeAttribute> {
        return match self.code_attr_index {
            Some(code_index) => {
                let attr_type = &self.attributes[code_index].attr_type;
                if let AttributeType::Code { code } = attr_type {
                    Some(code)
                } else {
                    None
                }
            }
            _ => None,
        };
    }
}

impl TryFromCp<&mut BytesMut> for MethodInfo {
    type Error = Error;

    fn try_from_cp(
        buf: &mut BytesMut,
        constant_pool: &ConstantPoolRef,
    ) -> Result<Self, Self::Error> {
        let access_flags = buf.get_u16();
        let name_index = buf.get_u16();
        let descriptor_index = buf.get_u16();
        let attribute_count = buf.get_u16();
        let mut attributes: Vec<Attribute> = vec![];
        let mut code_attr_index = None;
        for i in 0..attribute_count {
            let attribute = Attribute::try_from_cp(buf, constant_pool)?;
            let attr_type = &attribute.attr_type;
            if let AttributeType::Code { .. } = attr_type {
                code_attr_index = Some(i as usize);
            }
            attributes.push(attribute);
        }
        Ok(MethodInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
            code_attr_index,
        })
    }
}

impl<T> TryInto<&mut T, usize> for MethodInfo
where
    T: BufMut,
{
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
