#[macro_use]
pub mod macros;
pub mod basic_type;
pub mod classpath;
pub mod oops;
pub mod runtime;
pub mod stack;
pub mod types;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
