#![feature(asm, lang_items)]
#![crate_type="staticlib"]
#![no_std]

const GPFSEL3: u32 = 0x3F20_000C;
const GPFSEL4: u32 = 0x3F20_0010;
const GPSET1: u32 = 0x3F20_0020;
const GPCLR1: u32 = 0x3F20_002C;

const GPIO35: u32 = 1 << (35 - 32);
const GPIO47: u32 = 1 << (47 - 32);

extern {
    fn dummy();
}

fn sleep(times: u32) {
    for _ in 0..times {
        unsafe { dummy() };
    }
}

#[no_mangle]
pub extern fn rust_main() {
    let gpfsel3 = GPFSEL3 as *mut u32;
    let gpfsel4 = GPFSEL4 as *mut u32;
    let gpset1 = GPSET1 as *mut u32;
    let gpclr1 = GPCLR1 as *mut u32;

    let mut gpfsel3_val = unsafe { *gpfsel3 };
    gpfsel3_val &= !(7 << 15);
    gpfsel3_val |= 1 << 15;
    unsafe { *gpfsel3 = gpfsel3_val };

    let mut gpfsel4_val = unsafe { *gpfsel4 };
    gpfsel4_val &= !(7 << 21);
    gpfsel4_val |= 1 << 21;
    unsafe { *gpfsel4 = gpfsel4_val };

    loop {
        unsafe { *gpset1 = GPIO47 };
        unsafe { *gpclr1 = GPIO35 };
        sleep(100_000);
        unsafe { *gpclr1 = GPIO47 };
        unsafe { *gpset1 = GPIO35 };
        sleep(100_000);
    }
}

#[no_mangle]
pub unsafe fn __aeabi_unwind_cpp_pr0() {
    loop {}
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! {
    loop {}
}
