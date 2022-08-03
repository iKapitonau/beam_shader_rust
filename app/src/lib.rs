#![no_std]
#![no_main]

use ::common::common::env;
use ::common::common::*;
use ::common::*;

use core::mem::size_of_val;

type ActionFunc = fn(cid: ContractID);
type ActionsMap<'a> = &'a [(&'a str, ActionFunc)];

// MANAGER ACTIONS

fn on_action_create_contract(_unused: ContractID) {}

fn on_action_destroy_contract(_cid: ContractID) {}

fn on_action_view_contracts(_unused: ContractID) {
    env::enum_and_dump_contracts(&::common::SID);
}

fn on_action_view_logs(_cid: ContractID) {}

fn on_action_view_accounts(_cid: ContractID) {}

fn on_action_view_account(_cid: ContractID) {}

// MY_ACCOUNT ACTIONS
fn on_action_view(_cid: ContractID) {}

fn on_action_get_key(_cid: ContractID) {}

fn on_action_get_proof(_cid: ContractID) {}

fn on_action_deposit(_cid: ContractID) {}

fn on_action_withdraw(_cid: ContractID) {}

#[no_mangle]
#[allow(non_snake_case)]
fn Method_0() {}

#[no_mangle]
#[allow(non_snake_case)]
fn Method_1() {
    const INVALID_ROLE_ACTIONS: [(&str, ActionFunc); 0] = [];

    const VALID_MY_ACCOUNT_ACTIONS: [(&str, ActionFunc); 5] = [
        ("view\0", on_action_view),
        ("get_key\0", on_action_get_key),
        ("get_proof\0", on_action_get_proof),
        ("deposit\0", on_action_deposit),
        ("withdraw\0", on_action_withdraw),
    ];

    const VALID_MANAGER_ACTIONS: [(&str, ActionFunc); 6] = [
        ("create\0", on_action_create_contract),
        ("destroy\0", on_action_destroy_contract),
        ("view\0", on_action_view_contracts),
        ("view_logs\0", on_action_view_logs),
        ("view_accounts\0", on_action_view_accounts),
        ("view_account\0", on_action_view_account),
    ];

    const VALID_ROLES: [(&str, ActionsMap); 2] = [
        ("my_account\0", &VALID_MY_ACCOUNT_ACTIONS),
        ("manager\0", &VALID_MANAGER_ACTIONS),
    ];

    let mut role: [u8; 32] = Default::default();
    env::doc_get_text("role\0", &mut role, size_of_val(&role) as u32);

    let mut action_map: ActionsMap = &INVALID_ROLE_ACTIONS;
    for i in 0..VALID_ROLES.len() {
        if env::memcmp(
            &role,
            VALID_ROLES[i].0.as_ptr(),
            VALID_ROLES[i].0.len() as u32,
        ) == 0
        {
            action_map = VALID_ROLES[i].1;
            break;
        }
    }

    if action_map == &INVALID_ROLE_ACTIONS {
        env::doc_add_text("error\0", "Invalid role\0".as_ptr());
        return;
    }

    let mut action: [u8; 32] = Default::default();
    env::doc_get_text("action\0", &mut action, size_of_val(&action) as u32);

    for i in 0..action_map.len() {
        if env::memcmp(
            &action,
            action_map[i].0.as_ptr(),
            action_map[i].0.len() as u32,
        ) == 0
        {
            let mut cid: ContractID = Default::default();
            env::doc_get_blob("cid\0", &mut cid, size_of_val(&cid) as u32);
            action_map[i].1(cid);
            return;
        }
    }

    env::doc_add_text("error\0", "Invalid action\0".as_ptr());
}
