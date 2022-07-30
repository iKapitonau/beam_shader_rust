#![no_std]
#![no_main]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals, non_upper_case_globals)]

use common::root::Env;
use common::root::ContractID;
use common::root::FundsChange;
use common::root::SigRequest;
use common::*;

type ActionFunc = fn(cid: ContractID);
type ActionsMap<'a> = &'a [(&'a str, ActionFunc)];

fn on_action_create_contract(_unused: ContractID) {
    let params = InitialParams {
        state: 333,
    };
    let funds = FundsChange {
        m_Amount: 0,
        m_Aid: 0,
        m_Consume: 0,
    };
    let sig = SigRequest {
        m_pID: 0 as *const u32,
        m_nID: 0,
    };
    Env::GenerateKernel(&Default::default(), InitialParams::kMethod, &params, 4, &funds, 0, &sig, 0, "Create contract\0".as_ptr(), 0);
}

fn on_action_destroy_contract(cid: ContractID) {
}

fn on_action_view_contracts(_unused: ContractID) {
}

fn on_action_view_contract_params(cid: ContractID) {
}

fn on_action_send_msg(cid: ContractID) {
}

fn on_action_get_my_msg(cid: ContractID) {
}

#[no_mangle]
fn Method_0() {
}

#[no_mangle]
fn Method_1() {
    const INVALID_ROLE_ACTIONS: [(&str, ActionFunc); 0] = [];

    const VALID_USER_ACTIONS: [(&str, ActionFunc); 2] = [
        ("send_secret\0", on_action_send_msg),
        ("get_secret\0", on_action_get_my_msg)
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
    Env::DocGetText("role\0", &mut role, 32);

    let mut action_map: ActionsMap = &INVALID_ROLE_ACTIONS;
    for i in 0..VALID_ROLES.len() {
        if Env::Memcmp(&role, VALID_ROLES[i].0.as_ptr(), VALID_ROLES[i].0.len() as u32) == 0 {
            action_map = VALID_ROLES[i].1;
            break;
        }
    }

    if action_map == &INVALID_ROLE_ACTIONS {
        Env::DocAddText("error\0", "Invalid role\0".as_ptr());
        return;
    }

    let mut action: [u8; 32] = Default::default();
    Env::DocGetText("action\0", &mut action, 32);

    for i in 0..action_map.len() {
        if Env::Memcmp(&action, action_map[i].0.as_ptr(), action_map[i].0.len() as u32) == 0 {
            let mut cid: ContractID = [0; 32];
            Env::DocGetBlob("cid", &mut cid, 32);
            action_map[i].1(cid);
            return;
        }
    }

    Env::DocAddText("error\0", "Invalid action\0".as_ptr());
}
