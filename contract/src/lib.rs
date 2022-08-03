#![no_std]
#![no_main]

use ::common::common::env;
use ::common::common::*;
use ::common::*;

use core::mem::size_of_val;

fn load_account(key: &Key) -> Amount {
    let mut ret: Amount = Default::default();
    if env::load_var(
        &key,
        size_of_val(key) as u32,
        &mut ret,
        size_of_val(&ret) as u32,
        KeyTag::INTERNAL,
    ) != 0
    {
        ret
    } else {
        0
    }
}

fn save_account(key: &Key, amount: Amount) {
    env::emit_log(
        key,
        size_of_val(key) as u32,
        &amount,
        size_of_val(&amount) as u32,
        KeyTag::INTERNAL,
    );
    if amount > 0 {
        env::save_var(
            key,
            size_of_val(key) as u32,
            &amount,
            size_of_val(&amount) as u32,
            KeyTag::INTERNAL,
        );
    } else {
        env::del_var(key, size_of_val(key) as u32);
    }
}

#[no_mangle]
#[allow(non_snake_case)]
fn Ctor(_params: *const usize) {}

#[no_mangle]
#[allow(non_snake_case)]
fn Dtor(_params: *const usize) {}

#[no_mangle]
#[allow(non_snake_case)]
fn Method_2(params: &Deposit) {
    let mut total: Amount = load_account(&params.request.key);
    // Strict::Add
    total += params.request.amount;
    env::halt_if(total < params.request.amount);

    save_account(&params.request.key, total);
    env::funds_lock(params.request.key.aid, params.request.amount);
}

#[no_mangle]
#[allow(non_snake_case)]
fn Method_3(params: &Withdraw) {
    let mut total: Amount = load_account(&params.request.key);
    //Strict::Sub
    env::halt_if(total < params.request.amount);
    total -= params.request.amount;
    save_account(&params.request.key, total);
    env::funds_unlock(params.request.key.aid, params.request.amount);
    env::add_sig(&params.request.key.account);
}
