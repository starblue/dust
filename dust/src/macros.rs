#[macro_export]
macro_rules! default_handler {
    ($n:ident) => {
        #[linkage = "weak"]
        pub fn $n() {
            loop {}
        }
    };
}
