#[macro_export]
macro_rules! def_ptr {
    ($name: ident, $t: ty) => {
        pub type $name = Box<$t>;
    };
}

#[macro_export]
macro_rules! def_ref {
    ($name: ident, $t: ty) => {
        pub type $name = std::sync::Arc<Box<$t>>;
    }
}