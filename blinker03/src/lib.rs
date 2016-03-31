#![feature(asm, lang_items)]
#![no_std]

const GPFSEL2: u32 = 0x3F20_0008;
const GPSET0: u32 = 0x3F20_001C;
const GPCLR0: u32 = 0x3F20_0028;

const ARM_TIMER_CTL: u32 = 0x3F00_B408;
const ARM_TIMER_CNT: u32 = 0x3F00_B420;

const GPIO20: u32 = 1 << 20;
const GPIO21: u32 = 1 << 21;
const GPIO22: u32 = 1 << 22;

const TIMEOUT: u32 = 400_000;

#[no_mangle]
pub extern fn rust_main() {
    // Set the counter scalar to 0xF9. System clock freq (~250MHz) / 0xF9 = ~1 million ticks/sec
    write_gpio_register(ARM_TIMER_CTL, 0x00F9_0200);

    let mut gpfsel2_val = read_gpio_register(GPFSEL2);
    gpfsel2_val &= !(0x1FF as u32);
    gpfsel2_val |= 0x49;
    write_gpio_register(GPFSEL2, gpfsel2_val);

    let mut timer_state = read_gpio_register(ARM_TIMER_CNT);
    let mut blue_gpio = GPSET0;

    // Main blinky loop
    loop {
        write_gpio_register(GPSET0, GPIO20);
        write_gpio_register(GPCLR0, GPIO21);

        write_gpio_register(blue_gpio, GPIO22);
        blue_gpio = if blue_gpio == GPSET0 { GPCLR0 } else { GPSET0 };

        timer_state = wait_for_timeout(timer_state, TIMEOUT);

        write_gpio_register(GPCLR0, GPIO20);
        write_gpio_register(GPSET0, GPIO21);

        timer_state = wait_for_timeout(timer_state, TIMEOUT);
    }
}

fn wait_for_timeout(last_state: u32, timeout: u32) -> u32{
    loop {
        let current_timer_val = read_gpio_register(ARM_TIMER_CNT);
        if current_timer_val.wrapping_sub(last_state) >= timeout {
            break;
        }
    }
    last_state.wrapping_add(TIMEOUT)
}

fn write_gpio_register(address: u32, value: u32) {
    unsafe { *(address as *mut u32) = value };
}

fn read_gpio_register(address: u32) -> u32 {
    unsafe { *(address as *mut u32) }
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
