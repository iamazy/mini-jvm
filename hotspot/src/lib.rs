#[macro_use]
pub mod macros;
pub mod types;
pub mod stack;
pub mod oops;
pub mod classpath;
pub mod runtime;
pub mod basic_type;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
