#![feature(asm, lang_items)]
#![crate_type="staticlib"]
#![no_std]

const GPFSEL2: u32 = 0x3F20_0008;
const GPSET0: u32 = 0x3F20_001C;
const GPCLR0: u32 = 0x3F20_0028;
const SYSTEMTIMERCLO: u32 = 0x3F00_3004;

const GPIO20: u32 = 1 << 20;
const GPIO21: u32 = 1 << 21;

const TIMER_BIT: u32 = 0x0008_0000;

fn wait_for_timer_bit_state(bit: u32, state: u32) {
    loop {
        let timer_val = read_gpio_register(SYSTEMTIMERCLO as *mut u32);
        if (timer_val & bit) == state {
            return;
        }
    }
}

#[no_mangle]
pub extern fn rust_main() {
    let gpfsel2 = GPFSEL2 as *mut u32;
    let gpset0 = GPSET0 as *mut u32;
    let gpclr0 = GPCLR0 as *mut u32;

    let mut gpfsel2_val = read_gpio_register(gpfsel2);
    gpfsel2_val &= !(0b111111 as u32);
    gpfsel2_val |= 0b1001;
    write_gpio_register(gpfsel2, gpfsel2_val);

    loop {
        write_gpio_register(gpset0, GPIO20);
        write_gpio_register(gpclr0, GPIO21);

        wait_for_timer_bit_state(TIMER_BIT, TIMER_BIT);

        write_gpio_register(gpclr0, GPIO20);
        write_gpio_register(gpset0, GPIO21);

        wait_for_timer_bit_state(TIMER_BIT, 0);
    }
}

fn read_gpio_register(register: *mut u32) -> u32 {
    unsafe { *register }
}

fn write_gpio_register(register: *mut u32, value: u32) {
    unsafe { *register = value };
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
