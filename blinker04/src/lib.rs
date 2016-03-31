#![feature(asm, core_intrinsics, lang_items, naked_functions)]
#![no_std]

const GPFSEL2: u32 = 0x3F20_0008;
const GPSET0: u32 = 0x3F20_001C;
const GPCLR0: u32 = 0x3F20_0028;

const ARM_TIMER_LOD: u32 = 0x3F00_B400;
const ARM_TIMER_CTL: u32 = 0x3F00_B408;
const ARM_TIMER_RIS: u32 = 0x3F00_B410;
const ARM_TIMER_CLI: u32 = 0x3F00_B40C;
const ARM_TIMER_RLD: u32 = 0x3F00_B418;
const ARM_TIMER_DIV: u32 = 0x3F00_B41C;

const GPIO20: u32 = 1 << 20;
const GPIO21: u32 = 1 << 21;
const GPIO22: u32 = 1 << 22;

const TIMEOUT: u32 = 500_000;

use core::intrinsics::{volatile_store, volatile_load};

#[naked]
#[no_mangle]
pub extern fn rust_main() {
    // Set the counter pre-divider to 0xF9. System clock freq (~250MHz) / 0xF9 = ~1 million ticks/sec
    write_gpio_register(ARM_TIMER_CTL, 0x003E_0000);
    write_gpio_register(ARM_TIMER_LOD, TIMEOUT - 1);
    write_gpio_register(ARM_TIMER_RLD, TIMEOUT - 1);
    write_gpio_register(ARM_TIMER_DIV, 0x0000_00F9);
    write_gpio_register(ARM_TIMER_CLI, 0);
    write_gpio_register(ARM_TIMER_CTL, 0x003E_0082);

    let mut gpfsel2_val = read_gpio_register(GPFSEL2);
    gpfsel2_val &= !(0x1FF as u32);
    gpfsel2_val |= 0x49;
    write_gpio_register(GPFSEL2, gpfsel2_val);

    let mut blue_gpio = GPSET0;

    // Main blinky loop
    loop {
        write_gpio_register(blue_gpio, GPIO22);
        blue_gpio = if blue_gpio == GPSET0 { GPCLR0 } else { GPSET0 };

        write_gpio_register(GPSET0, GPIO20);
        write_gpio_register(GPCLR0, GPIO21);

        wait_for_timeout();

        write_gpio_register(GPCLR0, GPIO20);
        write_gpio_register(GPSET0, GPIO21);

        wait_for_timeout();
    }
}

fn wait_for_timeout() {
    loop {
        if read_gpio_register(ARM_TIMER_RIS) > 0 { break; }
    }
    write_gpio_register(ARM_TIMER_CLI, 0);
}

fn write_gpio_register(address: u32, value: u32) {
    // unsafe { *(address as *mut u32) = value };
    unsafe { volatile_store::<u32>(address as *mut u32, value) };
}

fn read_gpio_register(address: u32) -> u32 {
    // unsafe { *(address as *mut u32) }
    unsafe { volatile_load::<u32>(address as *mut u32) }
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
