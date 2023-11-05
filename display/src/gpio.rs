use core::intrinsics::{volatile_store, volatile_load};

const GPIO_BASE: usize = 0x3F00_0000;

pub struct GpioRegister {
    address: usize
}

impl GpioRegister {
    pub const fn new(address: usize) -> Self {
        // if !(address & GPIO_BASE == GPIO_BASE) {
        //     panic!("Address does not lie within periphal memory space");
        // }

        Self{address: address}
    }

    pub fn write(&self, value: usize) {
        unsafe {
            volatile_store::<usize>(self.address as *mut usize, value);
        }
    }

    pub fn read(&self) -> usize {
        unsafe {
            volatile_load::<usize>(self.address as *mut usize)
        }
    }
}

pub const IRQ_BASIC_ENABLE: GpioRegister =  GpioRegister::new(GPIO_BASE + 0x00_B218);

pub const ARM_TIMER_LOD: GpioRegister = GpioRegister::new(GPIO_BASE + 0x00_B400);
pub const ARM_TIMER_CTL: GpioRegister = GpioRegister::new(GPIO_BASE + 0x00_B408);
pub const ARM_TIMER_CLI: GpioRegister = GpioRegister::new(GPIO_BASE + 0x00_B40C);
pub const ARM_TIMER_RLD: GpioRegister = GpioRegister::new(GPIO_BASE + 0x00_B418);
pub const ARM_TIMER_DIV: GpioRegister = GpioRegister::new(GPIO_BASE + 0x00_B41C);
