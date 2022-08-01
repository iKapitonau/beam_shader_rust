#![no_std]
#![no_main]

use common::root::env;
use common::root::*;
use common::*;

use core::mem::size_of_val;

type ActionFunc = fn(cid: ContractID);
type ActionsMap<'a> = &'a [(&'a str, ActionFunc)];

fn on_action_create_contract(_unused: ContractID) {
    let params = InitialParams { state: 333 };
    let funds = FundsChange {
        amount: 0,
        aid: 0,
        consume: 0,
    };
    let sig = SigRequest {
        id_ptr: 0 as *const usize,
        id_size: 0,
    };
    env::generate_kernel(
        &Default::default(),
        InitialParams::METHOD,
        &params,
        size_of_val(&params) as u32,
        &funds,
        0,
        &sig,
        0,
        "Create contract\0".as_ptr(),
        0,
    );
}

fn on_action_destroy_contract(cid: ContractID) {
    let params = DtorParams {};
    let funds = FundsChange {
        amount: 0,
        aid: 0,
        consume: 0,
    };
    let sig = SigRequest {
        id_ptr: 0 as *const usize,
        id_size: 0,
    };
    env::generate_kernel(
        &cid,
        DtorParams::METHOD,
        &params,
        size_of_val(&params) as u32,
        &funds,
        0,
        &sig,
        0,
        "Destroy contract\0".as_ptr(),
        0,
    );
}

fn on_action_view_contracts(_unused: ContractID) {}

fn on_action_view_contract_params(_cid: ContractID) {}

fn on_action_send_msg(cid: ContractID) {
    let funds = FundsChange {
        amount: 0,
        aid: 0,
        consume: 0,
    };
    let sig = SigRequest {
        id_ptr: 0 as *const usize,
        id_size: 0,
    };
    let mut key: u32 = Default::default();
    env::doc_get_num32("key\0", &mut key);
    let mut secret: u32 = Default::default();
    env::doc_get_num32("secret\0", &mut secret);
    let params = SendMsgParams { key, secret };
    env::generate_kernel(
        &cid,
        SendMsgParams::METHOD,
        &params,
        size_of_val(&params) as u32,
        &funds,
        0,
        &sig,
        0,
        "Send secret\0".as_ptr(),
        0,
    );
}

fn on_action_get_my_msg(cid: ContractID) {
    let mut key_u32: u32 = Default::default();
    env::doc_get_num32("key\0", &mut key_u32);

    let key = env::Key::<u32> {
        prefix: env::KeyPrefix {
            cid,
            tag: KeyTag::INTERNAL,
        },
        key_in_contract: key_u32,
    };
    let mut secret: u32 = Default::default();
    env::VarReader::read(&key, &mut secret);
    env::doc_add_num32("Your secret:\0", secret);
}

#[no_mangle]
#[allow(non_snake_case)]
fn Method_0() {}

#[no_mangle]
#[allow(non_snake_case)]
fn Method_1() {
    const INVALID_ROLE_ACTIONS: [(&str, ActionFunc); 0] = [];

    const VALID_USER_ACTIONS: [(&str, ActionFunc); 2] = [
        ("send_secret\0", on_action_send_msg),
        ("get_secret\0", on_action_get_my_msg),
    ];

    const VALID_MANAGER_ACTIONS: [(&str, ActionFunc); 4] = [
        ("create_contract\0", on_action_create_contract),
        ("destroy_contract\0", on_action_destroy_contract),
        ("view_contracts\0", on_action_view_contracts),
        ("view_contract_params\0", on_action_view_contract_params),
    ];

    const VALID_ROLES: [(&str, ActionsMap); 2] = [
        ("user\0", &VALID_USER_ACTIONS),
        ("manager\0", &VALID_MANAGER_ACTIONS),
    ];

    let mut role: [u8; 32] = Default::default();
    env::doc_get_text("role\0", &mut role, 32);

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
    env::doc_get_text("action\0", &mut action, 32);

    for i in 0..action_map.len() {
        if env::memcmp(
            &action,
            action_map[i].0.as_ptr(),
            action_map[i].0.len() as u32,
        ) == 0
        {
            let mut cid: ContractID = [0; 32];
            env::doc_get_blob("cid\0", &mut cid, 32);
            action_map[i].1(cid);
            return;
        }
    }

    env::doc_add_text("error\0", "Invalid action\0".as_ptr());
}
