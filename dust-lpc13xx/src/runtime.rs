#[cfg(not(test))]
#[lang = "panic_fmt"]
fn rust_begin_panic(
    _msg: ::core::fmt::Arguments,
    _file: &'static str,
    _line: u32,
) -> ! {
    loop {}
}
