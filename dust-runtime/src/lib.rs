#![feature(panic_handler)]
#![no_std]

#[cfg(not(any(unix, windows)))]
use core::panic::PanicInfo;

#[cfg(not(any(unix, windows)))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern "C" {
    static __data_load__: u32;
    static mut __data_start__: u32;
    static __data_end__: u32;

    static mut __bss_start__: u32;
    static __bss_end__: u32;
}

extern "C" {
    fn main();
}

unsafe fn init_data(load: *const u32, start: *mut u32, end: *const u32) {
    let mut s = load;
    let mut d = start;
    while d < (end as *mut u32) {
        *d = *s;
        s = s.offset(1);
        d = d.offset(1);
    }
}

unsafe fn init_bss(start: *mut u32, end: *const u32) {
    let mut d = start as *mut u32;
    while d < (end as *mut u32) {
        *d = 0;
        d = d.offset(1);
    }
}

pub unsafe fn crt0() -> ! {
    init_data(&__data_load__, &mut __data_start__, &__data_end__);
    init_bss(&mut __bss_start__, &__bss_end__);

    main();
    loop {}
}
