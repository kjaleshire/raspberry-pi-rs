use super::gpio;

const PM_RSTC: gpio::GpioRegister = gpio::GpioRegister::new(0x3F10_001C);
const PM_WDOG: gpio::GpioRegister = gpio::GpioRegister::new(0x3F10_0024);

const PM_PASSWORD: u32 =              0x5A00_0000;
const PM_WDOG_TIME_SET: u32 =         0x000F_FFFF;
const PM_RSTC_WRCFG_CLR: u32 =        0xFFFF_FFCF;
const PM_RSTC_WRCFG_FULL_RESET: u32 = 0x0000_0020;
const PM_RSTC_RESET: u32 =            0x0000_0102;

pub fn start(timeout: u32) {
    let timer_val = PM_PASSWORD | (timeout & PM_WDOG_TIME_SET);

    let reset_val = PM_RSTC.read();
    let new_reset_val = PM_PASSWORD | (reset_val & PM_RSTC_WRCFG_CLR) | PM_RSTC_WRCFG_FULL_RESET;

    PM_WDOG.write(timer_val);
    PM_RSTC.write(new_reset_val);
}

#[allow(dead_code)]
pub fn stop() {
    PM_RSTC.write(PM_PASSWORD | PM_RSTC_RESET);
}

pub fn remaining_time() -> u32 {
    PM_WDOG.read() & PM_WDOG_TIME_SET
}
