use core::intrinsics::{volatile_store, volatile_load};

const GPIO_BASE: u32 = 0x3F00_0000;

pub struct GpioRegister {
    address: u32
}

impl GpioRegister {
    pub const fn new(address: u32) -> Self {
        // if !(address & GPIO_BASE == GPIO_BASE) {
        //     panic!("Address does not lie within periphal memory space");
        // }

        Self{address: address}
    }

    pub fn write(&self, value: u32) {
        unsafe {
            volatile_store::<u32>(self.address as *mut u32, value);
        }
    }

    pub fn read(&self) -> u32 {
        unsafe {
            volatile_load::<u32>(self.address as *mut u32)
        }
    }
}

pub const IRQ_BASIC_ENABLE: GpioRegister =  GpioRegister::new(GPIO_BASE + 0x00_B218);

pub const ARM_TIMER_LOD: GpioRegister = GpioRegister::new(GPIO_BASE + 0x00_B400);
pub const ARM_TIMER_CTL: GpioRegister = GpioRegister::new(GPIO_BASE + 0x00_B408);
pub const ARM_TIMER_CLI: GpioRegister = GpioRegister::new(GPIO_BASE + 0x00_B40C);
pub const ARM_TIMER_RLD: GpioRegister = GpioRegister::new(GPIO_BASE + 0x00_B418);
pub const ARM_TIMER_DIV: GpioRegister = GpioRegister::new(GPIO_BASE + 0x00_B41C);
