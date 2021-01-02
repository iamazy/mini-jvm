#[derive(Debug, Clone)]
pub enum BasicType {
    BOOLEAN = 4,
    CHAR = 5,
    FLOAT = 6,
    DOUBLE = 7,
    BYTE = 8,
    SHORT = 9,
    INT = 10,
    LONG = 11,
    OBJECT = 12,
    ARRAY = 13,
    VOID = 14,
}

impl From<u8> for BasicType {
    fn from(value: u8) -> Self {
        match value {
            b'Z' => BasicType::BOOLEAN,
            b'C' => BasicType::CHAR,
            b'F' => BasicType::FLOAT,
            b'D' => BasicType::DOUBLE,
            b'B' => BasicType::BYTE,
            b'S' => BasicType::SHORT,
            b'I' => BasicType::INT,
            b'J' => BasicType::LONG,
            b'L' => BasicType::OBJECT,
            b'[' => BasicType::ARRAY,
            b'V' => BasicType::VOID,
            v => {
                let s = [v];
                let s = String::from_utf8_lossy(&s);
                unreachable!("Unknown BasicType = {}", s)
            }
        }
    }
}

impl Into<u8> for BasicType {
    fn into(self) -> u8 {
        match self {
            BasicType::BOOLEAN => b'Z',
            BasicType::CHAR => b'C',
            BasicType::FLOAT => b'F',
            BasicType::DOUBLE => b'D',
            BasicType::BYTE => b'B',
            BasicType::SHORT => b'S',
            BasicType::INT => b'I',
            BasicType::LONG => b'J',
            BasicType::OBJECT => b'L',
            BasicType::ARRAY => b'[',
            BasicType::VOID => b'V',
        }
    }
}

impl BasicType {
    pub fn get_primitive_name(&self) -> &'static [u8] {
        match *self {
            BasicType::BOOLEAN => b"boolean",
            BasicType::CHAR => b"char",
            BasicType::FLOAT => b"float",
            BasicType::DOUBLE => b"double",
            BasicType::BYTE => b"byte",
            BasicType::SHORT => b"short",
            BasicType::INT => b"int",
            BasicType::LONG => b"long",
            BasicType::OBJECT | BasicType::ARRAY | BasicType::VOID => unreachable!(),
        }
    }
}
