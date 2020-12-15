#[derive(Debug, Clone)]
pub enum Error {

    InvalidLength,

    InvalidString(String),

    InvalidConstantTag(u8)
}