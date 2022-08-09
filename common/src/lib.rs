#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

include!("bvm_bindings.rs");
include!("contract_sid.rs");

use crate::root::*;

#[repr(C, packed(1))]
pub struct CtorParams {}

#[repr(C, packed(1))]
pub struct DtorParams {}

impl CtorParams {
    pub const METHOD: u32 = 0;
}

impl DtorParams {
    pub const METHOD: u32 = 1;
}
