#![no_std]
#![no_main]

use common::root::Env;
use common::root::KeyTag;
use common::*;

use core::mem::size_of_val;

#[no_mangle]
fn Ctor(params: &InitialParams) {
}

#[no_mangle]
fn Dtor(params: &DtorParams) {
}

#[no_mangle]
fn Method_2(params: &SendMsgParams) {
    Env::SaveVar(&params.key, size_of_val(&params.key) as u32, &params.secret, size_of_val(&params.secret) as u32, KeyTag::kInternal);
}
