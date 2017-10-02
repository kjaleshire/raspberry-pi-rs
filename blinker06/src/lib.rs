#![feature(core_intrinsics, lang_items)]
#![no_std]

mod gpio;
mod watchdog;

use core::intrinsics::{volatile_store, volatile_load};

const GPFSEL2: u32 = 0x3F20_0008;
const GPSET0: u32 =  0x3F20_001C;
const GPCLR0: u32 =  0x3F20_0028;

const GPIO20: u32 = 1 << 20;
const GPIO21: u32 = 1 << 21;
const GPIO22: u32 = 1 << 22;

const IRQ_BASIC_ENABLE: u32 =  0x3F00_B218;

const ARM_TIMER_LOD: u32 = 0x3F00_B400;
const ARM_TIMER_CTL: u32 = 0x3F00_B408;
const ARM_TIMER_CLI: u32 = 0x3F00_B40C;
const ARM_TIMER_RLD: u32 = 0x3F00_B418;
const ARM_TIMER_DIV: u32 = 0x3F00_B41C;

const SHORT_TIMEOUT: u32 = 500_000;

static mut STATE_COUNTER: u32 = 0;

extern {
    fn enable_irq();
}

#[no_mangle]
pub extern fn rust_main() {
    gpio::write_register(ARM_TIMER_LOD, SHORT_TIMEOUT - 1);
    gpio::write_register(ARM_TIMER_RLD, SHORT_TIMEOUT - 1);
    // // Set the timer pre-divider to 0xF9. System clock freq (~250MHz) / 0xF9 = ~1 million ticks/sec
    gpio::write_register(ARM_TIMER_DIV, 0x0000_00F9);
    gpio::write_register(ARM_TIMER_CLI, 0);
    gpio::write_register(ARM_TIMER_CTL, 0x003E_00A2);

    gpio::write_register(IRQ_BASIC_ENABLE, 0x1);

    initialize_leds();

    unsafe { enable_irq(); }

    watchdog::start(0x000F_FFFF);

    loop {
        if watchdog::remaining_time() < 0x67697 {
            gpio::write_register(GPSET0, GPIO22);
            break;
        }
    }

    loop {}
}

#[no_mangle]
pub extern fn rust_irq_handler() {
    let state_counter = unsafe { volatile_load::<u32>(&STATE_COUNTER as *const u32 as *mut u32) };

    if state_counter & 0x1 == 0 {
        gpio::write_register(GPSET0, GPIO20);
        gpio::write_register(GPCLR0, GPIO21);
    } else {
        gpio::write_register(GPCLR0, GPIO20);
        gpio::write_register(GPSET0, GPIO21);
    }

    unsafe {
        volatile_store(&STATE_COUNTER as *const u32 as *mut u32, state_counter.wrapping_add(1));
    }

    gpio::write_register(ARM_TIMER_CLI, 0);
}

fn initialize_leds() {
    let mut gpfsel2_val = gpio::read_register(GPFSEL2);
    gpfsel2_val &= !0x1FF; // mask out other GPIO pins
    gpfsel2_val |= 0x49; // 0b01001001
    gpio::write_register(GPFSEL2, gpfsel2_val);
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
