#![feature(core_intrinsics, lang_items, const_fn, compiler_builtins_lib)]
#![no_std]

extern crate compiler_builtins;
extern crate metalpi;

mod gpio;
mod watchdog;

use metalpi::gpio as gpio_n;
use core::intrinsics::{volatile_store, volatile_load};

const SHORT_TIMEOUT: u32 = 500_000;

static mut STATE_COUNTER: u32 = 0;

extern {
    fn enable_irq();
}

#[no_mangle]
pub extern fn rust_main() {
    gpio::ARM_TIMER_LOD.write(SHORT_TIMEOUT - 1);
    gpio::ARM_TIMER_RLD.write(SHORT_TIMEOUT - 1);
    // // Set the timer pre-divider to 0xF9. System clock freq (~250MHz) / 0xF9 = ~1 million ticks/sec
    gpio::ARM_TIMER_DIV.write(0x0000_00F9);
    gpio::ARM_TIMER_CLI.write(0);
    gpio::ARM_TIMER_CTL.write(0x003E_00A2);

    gpio::IRQ_BASIC_ENABLE.write(0x1);

    initialize_leds();

    unsafe { enable_irq(); }

    watchdog::start(0x000F_FFFF);

    loop {
        if watchdog::remaining_time() < 0x67697 {
            gpio_n::set_pin(22);
            break;
        }
    }

    loop {}
}

#[no_mangle]
pub extern fn rust_irq_handler() {
    let state_counter = unsafe { volatile_load::<u32>(&STATE_COUNTER as *const u32 as *mut u32) };

    if state_counter & 0x1 == 0 {
        gpio_n::set_pin(20);
        gpio_n::clear_pin(21);
    } else {
        gpio_n::clear_pin(20);
        gpio_n::set_pin(21);
    }

    unsafe {
        volatile_store(&STATE_COUNTER as *const u32 as *mut u32, state_counter.wrapping_add(1));
    }

    gpio::ARM_TIMER_CLI.write(0);
}

fn initialize_leds() {
    gpio_n::select_fn(20, gpio_n::PinFn::Output);
    gpio_n::select_fn(21, gpio_n::PinFn::Output);
    gpio_n::select_fn(22, gpio_n::PinFn::Output);
}

#[no_mangle]
pub unsafe fn __aeabi_unwind_cpp_pr0() { loop {} }

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr1() { loop {} }

#[no_mangle]
pub fn __aeabi_ul2f(x: u64) -> f32 { x as f32 }

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! { loop {} }
