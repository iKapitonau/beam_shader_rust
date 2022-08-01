#![no_std]
#![no_main]

include!("bvm_bindings.rs");

use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

#[repr(packed(1))]
pub struct InitialParams {
    pub state: u32,
}

#[repr(packed(1))]
pub struct DtorParams {
}

#[repr(packed(1))]
pub struct SendMsgParams {
    pub key: u32,
    pub secret: u32,
}

impl InitialParams {
    pub const kMethod: u32 = 0;
}

impl DtorParams {
    pub const kMethod: u32 = 1;
}

impl SendMsgParams {
    pub const kMethod: u32 = 2;
}
