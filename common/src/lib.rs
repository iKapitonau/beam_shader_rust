#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

include!("bvm_bindings.rs");
include!("contract_sid.rs");

use crate::common::*;

#[repr(C, packed(1))]
pub struct Key {
    pub account: PubKey,
    pub aid: AssetID,
}

#[repr(C, packed(1))]
pub struct Request {
    pub key: Key,
    pub amount: Amount,
}

#[repr(C, packed(1))]
pub struct Deposit {
    pub request: Request,
}

#[repr(C, packed(1))]
pub struct Withdraw {
    pub request: Request,
}

impl Deposit {
    pub const METHOD: u32 = 2;
}

impl Withdraw {
    pub const METHOD: u32 = 3;
}
