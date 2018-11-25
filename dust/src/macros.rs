#[macro_export]
macro_rules! default_handler {
    ($n:ident) => {
        #[linkage = "weak"]
        #[no_mangle]
        pub fn $n() {
            loop {}
        }
    };
}
