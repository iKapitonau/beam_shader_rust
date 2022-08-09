#![no_std]
#![no_main]

use common::root::env;
use common::root::*;
use common::*;

use core::mem::size_of_val;

type ActionFunc = fn(cid: ContractID);
type ActionsMap<'a> = &'a [(&'a str, ActionFunc)];

// MANAGER ACTIONS

fn on_action_create_contract(_unused: ContractID) {
    let args = CtorParams {};
    env::generate_kernel(
        0 as *const ContractID,
        CtorParams::METHOD,
        &args,
        size_of_val(&args) as u32,
        0 as *const FundsChange,
        0,
        0 as *const SigRequest,
        0,
        "Create contract\0".as_ptr(),
        0,
    );
}

fn on_action_destroy_contract(cid: ContractID) {
    let args = DtorParams {};
    env::generate_kernel(
        &cid,
        DtorParams::METHOD,
        &args,
        size_of_val(&args) as u32,
        0 as *const FundsChange,
        0,
        0 as *const SigRequest,
        0,
        "Destroy contract\0".as_ptr(),
        0,
    );
}

fn on_action_view_contracts(_unused: ContractID) {
    env::enum_and_dump_contracts(&::common::SID);
}

#[no_mangle]
#[allow(non_snake_case)]
fn Method_0() {
    env::doc_add_group("\0");
    env::doc_add_group("roles\0");
    env::doc_add_group("manager\0");

    env::doc_add_group("create\0");
    env::doc_close_group(); // create

    env::doc_add_group("destroy\0");
    env::doc_add_text("cid\0", "ContractID\0".as_ptr());
    env::doc_close_group(); // destroy

    env::doc_add_group("view\0");
    env::doc_close_group(); // view

    env::doc_close_group(); // manager
    env::doc_close_group(); // roles
    env::doc_close_group(); // \0
}

#[no_mangle]
#[allow(non_snake_case)]
fn Method_1() {
    const INVALID_ROLE_ACTIONS: [(&str, ActionFunc); 0] = [];

    //const VALID_MY_ACCOUNT_ACTIONS: [(&str, ActionFunc); 0] = [
    //];

    const VALID_MANAGER_ACTIONS: [(&str, ActionFunc); 3] = [
        ("create\0", on_action_create_contract),
        ("destroy\0", on_action_destroy_contract),
        ("view\0", on_action_view_contracts),
    ];

    const VALID_ROLES: [(&str, ActionsMap); 1] = [
        // ("my_account\0", &VALID_MY_ACCOUNT_ACTIONS),
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
