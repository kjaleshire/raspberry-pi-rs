#![feature(asm, core_intrinsics, lang_items, naked_functions)]
#![no_std]

const GPFSEL2: u32 = 0x3F20_0008;
const GPSET0: u32 = 0x3F20_001C;
const GPCLR0: u32 = 0x3F20_0028;

const GPIO20: u32 = 1 << 20;
const GPIO21: u32 = 1 << 21;
const GPIO22: u32 = 1 << 22;

const IRQ_BASIC_PENDING: u32 = 0x3F00_B200;
const IRQ_BASIC_ENABLE: u32 =  0x3F00_B218;
const IRQ_BASIC_DISABLE: u32 = 0x3F00_B224;

const ARM_TIMER_LOD: u32 = 0x3F00_B400;
const ARM_TIMER_CTL: u32 = 0x3F00_B408;
const ARM_TIMER_CLI: u32 = 0x3F00_B40C;
const ARM_TIMER_MIS: u32 = 0x3F00_B414;
const ARM_TIMER_RLD: u32 = 0x3F00_B418;
const ARM_TIMER_DIV: u32 = 0x3F00_B41C;

const LONG_TIMEOUT: u32 = 2_000_000;
const MEDIUM_TIMEOUT: u32 = 1_000_000;
const SHORT_TIMEOUT: u32 = 500_000;

static mut STATE_COUNTER: u32 = 0;

use core::intrinsics::{volatile_store, volatile_load};

extern {
    fn enable_irq();
}

#[naked]
#[no_mangle]
pub extern fn rust_main() {
    // Disable the ARM Timer IRQ
    write_gpio_register(IRQ_BASIC_DISABLE, 0x1);

    write_gpio_register(ARM_TIMER_LOD, LONG_TIMEOUT - 1);
    write_gpio_register(ARM_TIMER_RLD, LONG_TIMEOUT - 1);
    // Set the timer pre-divider to 0xF9. System clock freq (~250MHz) / 0xF9 = ~1 million ticks/sec
    write_gpio_register(ARM_TIMER_DIV, 0x0000_00F9);
    write_gpio_register(ARM_TIMER_CLI, 0);
    write_gpio_register(ARM_TIMER_CTL, 0x003E_00A2);

    let mut gpfsel2_val = read_gpio_register(GPFSEL2);
    gpfsel2_val &= !0x1FF;
    gpfsel2_val |= 0x49;
    write_gpio_register(GPFSEL2, gpfsel2_val);

    // Normal polling loop (blinker04)
    for _ in 0..2 {
        write_gpio_register(GPSET0, GPIO20);
        write_gpio_register(GPCLR0, GPIO21);

        wait_for_masked_irq_interrupt();

        write_gpio_register(GPCLR0, GPIO20);
        write_gpio_register(GPSET0, GPIO21);

        wait_for_masked_irq_interrupt();
    }

    // Interrupt Basic status register polling
    write_gpio_register(ARM_TIMER_LOD, MEDIUM_TIMEOUT - 1);
    write_gpio_register(ARM_TIMER_RLD, MEDIUM_TIMEOUT - 1);
    write_gpio_register(ARM_TIMER_CLI, 0);

    // Enable the ARM Timer IRQ bit
    write_gpio_register(IRQ_BASIC_ENABLE, 0x1);

    for _ in 0..3 {
        write_gpio_register(GPSET0, GPIO20);
        write_gpio_register(GPCLR0, GPIO21);

        wait_for_basic_irq_interrupt();

        write_gpio_register(GPCLR0, GPIO20);
        write_gpio_register(GPSET0, GPIO21);

        wait_for_basic_irq_interrupt();
    }

    write_gpio_register(ARM_TIMER_LOD, SHORT_TIMEOUT - 1);
    write_gpio_register(ARM_TIMER_RLD, SHORT_TIMEOUT - 1);

    // Enable IRQ's, the ARM Timer enable bit is already set
    unsafe { enable_irq(); }

    loop {}
}

fn wait_for_masked_irq_interrupt() {
    loop {
        if read_gpio_register(ARM_TIMER_MIS) > 0 { break; }
    }
    write_gpio_register(ARM_TIMER_CLI, 0);
}

fn wait_for_basic_irq_interrupt() {
    loop {
        if (read_gpio_register(IRQ_BASIC_PENDING) & 0x1) > 0 { break; }
    }
    write_gpio_register(ARM_TIMER_CLI, 0);
}

#[naked]
#[no_mangle]
pub extern fn rust_irq_handler() {
    let state_counter = unsafe {
        volatile_load::<u32>(&STATE_COUNTER as *const u32 as *mut u32)
    };

    if state_counter % 2 == 0 {
        write_gpio_register(GPSET0, GPIO20);
        write_gpio_register(GPCLR0, GPIO21);
        if state_counter % 4 == 0 {
            write_gpio_register(GPSET0, GPIO22);
        } else {
            write_gpio_register(GPCLR0, GPIO22);
        }
    } else {
        write_gpio_register(GPCLR0, GPIO20);
        write_gpio_register(GPSET0, GPIO21);
    }

    unsafe {
        let new_state = state_counter.wrapping_add(1);
        volatile_store(&STATE_COUNTER as *const u32 as *mut u32, new_state);
    };

    write_gpio_register(ARM_TIMER_CLI, 0);
}

fn write_gpio_register(address: u32, value: u32) {
    unsafe {
        volatile_store::<u32>(address as *mut u32, value);
    }
}

fn read_gpio_register(address: u32) -> u32 {
    unsafe {
        volatile_load::<u32>(address as *mut u32)
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
