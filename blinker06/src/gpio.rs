use core::intrinsics::{volatile_store, volatile_load};

const GPIO_BASE: u32 = 0x3F00_0000;

pub fn write_register(address: u32, value: u32) {
    assert!(address & GPIO_BASE == GPIO_BASE);

    unsafe {
        volatile_store::<u32>(address as *mut u32, value);
    }
}

pub fn read_register(address: u32) -> u32 {
    assert!(address & GPIO_BASE == GPIO_BASE);

    unsafe {
        volatile_load::<u32>(address as *mut u32)
    }
}
