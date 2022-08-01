#![no_std]
#![no_main]

use common::root::env;
use common::root::*;
use common::*;

use core::mem::size_of_val;

#[no_mangle]
#[allow(non_snake_case)]
fn Ctor(_params: &InitialParams) {}

#[no_mangle]
#[allow(non_snake_case)]
fn Dtor(_params: &DtorParams) {}

#[no_mangle]
#[allow(non_snake_case)]
fn Method_2(params: &SendMsgParams) {
    let key: u32 = params.key;
    let secret: u32 = params.secret;
    env::save_var(
        &key,
        size_of_val(&key) as u32,
        &secret,
        size_of_val(&secret) as u32,
        KeyTag::INTERNAL,
    );
}
