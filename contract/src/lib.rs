#![no_std]
#![no_main]

use common::root::Env;
use common::root::KeyTag;
use common::*;

#[no_mangle]
fn Ctor(params: &InitialParams) {
}

#[no_mangle]
fn Dtor(params: &DtorParams) {
}

#[no_mangle]
fn Method_2(params: &SendMsgParams) {
    let key: u32 = params.key;
    let key_size: u32 = 4;
    let val: u32 = params.secret;
    let val_size: u32 = 4;
    let tag = KeyTag::kInternal;
    Env::SaveVar(&key, key_size, &val, val_size, tag);
}
