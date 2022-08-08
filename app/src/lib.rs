#![no_std]
#![no_main]

use common::root::env;
use common::root::*;
use common::*;

use core::mem::size_of_val;

type ActionFunc = fn(cid: ContractID);
type ActionsMap<'a> = &'a [(&'a str, ActionFunc)];

type KeyAccount = env::Key<Key>;

#[repr(C, packed(1))]
struct MyAccountID {
    pub cid: ContractID,
    pub ctx: u8,
}

fn my_account_move(
    deposit: bool,
    cid: &ContractID,
    foreign_key: &PubKey,
    cosigner: u32,
    amount: Amount,
    aid: AssetID,
) {
    if amount == 0 {
        env::doc_add_text("error\0", "amount must be non-zero\0".as_ptr());
        return;
    }
    let mut args = Request {
        amount,
        key: Key {
            account: Default::default(),
            aid,
        },
    };

    let my_id = MyAccountID { cid: *cid, ctx: 0 };
    let kid = KeyID {
        id_ptr: &my_id as *const MyAccountID as *const usize,
        id_size: size_of_val(&my_id) as u32,
    };

    let mut my_pk = Default::default();
    let is_multisig: bool = env::mem_is_0(foreign_key, size_of_val(foreign_key) as u32) == 0;

    if is_multisig {
        let mut p0: secp::Point = Default::default();
        let mut p1: secp::Point = Default::default();

        if !p1.import(foreign_key) {
            env::doc_add_text("error\0", "bad foreign key\0".as_ptr());
            return;
        }
        kid.get_pk(&mut p0);
        p0.export(&mut my_pk);
        p0 += p1;
        p0.export(&mut args.key.account);
    } else {
        kid.get_pk(&mut args.key.account);
    }

    let fc = FundsChange {
        amount,
        aid,
        consume: deposit as u8,
    };

    if deposit {
        env::generate_kernel(
            cid,
            Deposit::METHOD,
            &args,
            size_of_val(&args) as u32,
            &fc,
            1,
            0 as *const SigRequest,
            0,
            "Deposit to Vault\0".as_ptr(),
            0,
        );
    } else {
        if is_multisig {
            // TODO
            if cosigner != 0 {
            } else {
            }
        } else {
            env::generate_kernel(
                cid,
                Withdraw::METHOD,
                &args,
                size_of_val(&args) as u32,
                &fc,
                1,
                &kid,
                1,
                "Withdraw from Vault\0".as_ptr(),
                0,
            );
        }
    }
}

fn derive_my_pk(pubkey: &mut PubKey, cid: &ContractID) {
    let my_id = MyAccountID { cid: *cid, ctx: 0 };
    env::derive_pk(pubkey, &my_id, size_of_val(&my_id) as u32);
}

fn dump_accounts(r: &mut env::VarReader) {
    env::doc_add_array("accounts\0");
    loop {
        let mut key = KeyAccount {
            prefix: Default::default(),
            key_in_contract: Key {
                account: Default::default(),
                aid: Default::default(),
            },
        };
        let mut amount: Amount = Default::default();

        if !r.move_next_t(&mut key, &mut amount) {
            break;
        }
        env::doc_add_group("\0");
        env::doc_add_blob(
            "Account\0",
            &key.key_in_contract.account,
            size_of_val(&key.key_in_contract.account) as u32,
        );
        env::doc_add_num32("AssetID\0", key.key_in_contract.aid);
        env::doc_add_num64("Amount\0", amount);
        env::doc_close_group();
    }
    env::doc_close_array();
}

fn dump_account(pubkey: &PubKey, cid: &ContractID) {
    let k0 = KeyAccount {
        prefix: env::KeyPrefix {
            cid: *cid,
            ..Default::default()
        },
        key_in_contract: Key {
            account: *pubkey,
            aid: Default::default(),
        },
    };
    let k1 = KeyAccount {
        key_in_contract: Key {
            aid: AssetID::MAX,
            ..k0.key_in_contract
        },
        prefix: env::KeyPrefix { ..k0.prefix },
    };
    let mut r = env::VarReader::new(&k0, &k1);
    dump_accounts(&mut r);
}

// MANAGER ACTIONS

fn on_action_create_contract(_unused: ContractID) {
    env::generate_kernel(
        0 as *const ContractID,
        0,
        0 as *const usize,
        0,
        0 as *const FundsChange,
        0,
        0 as *const SigRequest,
        0,
        "Create Vault contract\0".as_ptr(),
        0,
    );
}

fn on_action_destroy_contract(cid: ContractID) {
    env::generate_kernel(
        &cid,
        1,
        0 as *const usize,
        0,
        0 as *const FundsChange,
        0,
        0 as *const SigRequest,
        0,
        "Destroy Vault contract\0".as_ptr(),
        0,
    );
}

fn on_action_view_contracts(_unused: ContractID) {
    env::enum_and_dump_contracts(&::common::SID);
}

fn on_action_view_logs(cid: ContractID) {
    let k0 = KeyAccount {
        prefix: env::KeyPrefix {
            cid,
            ..Default::default()
        },
        key_in_contract: Key {
            account: Default::default(),
            aid: Default::default(),
        },
    };
    let k1 = KeyAccount {
        prefix: env::KeyPrefix { ..k0.prefix },
        key_in_contract: Key {
            account: PubKey {
                x: [0xff; 32],
                y: 0xff,
            },
            aid: AssetID::MAX,
        },
    };

    let mut lr: env::LogReader =
        env::LogReader::new(&k0, &k1, 0 as *const HeightPos, 0 as *const HeightPos);
    env::doc_add_array("logs\0");
    loop {
        let mut key = KeyAccount {
            prefix: env::KeyPrefix {
                cid,
                ..Default::default()
            },
            key_in_contract: Key {
                account: Default::default(),
                aid: Default::default(),
            },
        };
        let mut val: Amount = Default::default();

        if !lr.move_next_t(&mut key, &mut val) {
            break;
        }

        env::doc_add_group("\0");
        env::doc_add_num64("Height\0", lr.pos.height);
        env::doc_add_num32("Pos\0", lr.pos.pos);
        env::doc_add_blob(
            "Account\0",
            &key.key_in_contract.account,
            size_of_val(&key.key_in_contract.account) as u32,
        );
        env::doc_add_num32("AssetID\0", key.key_in_contract.aid);
        env::doc_add_num64("Amount\0", val);
        env::doc_close_group();
    }
    env::doc_close_array();
}

fn on_action_view_accounts(cid: ContractID) {
    let k0 = env::KeyPrefix {
        cid,
        ..Default::default()
    };
    let k1 = env::KeyPrefix {
        cid,
        tag: KeyTag::INTERNAL + 1,
    };
    let mut r = env::VarReader::new(&k0, &k1);
    dump_accounts(&mut r);
}

fn on_action_view_account(cid: ContractID) {
    let mut pubkey: PubKey = Default::default();
    env::doc_get_blob("pubkey\0", &mut pubkey, size_of_val(&pubkey) as u32);
    dump_account(&pubkey, &cid);
}

// MY_ACCOUNT ACTIONS
fn on_action_view(cid: ContractID) {
    let mut pubkey: PubKey = Default::default();
    derive_my_pk(&mut pubkey, &cid);
    dump_account(&pubkey, &cid);
}

fn on_action_get_key(cid: ContractID) {
    let mut pubkey: PubKey = Default::default();
    derive_my_pk(&mut pubkey, &cid);
    env::doc_add_blob("key\0", &mut pubkey, size_of_val(&pubkey) as u32);
}

fn on_action_get_proof(cid: ContractID) {
    let mut aid: AssetID = Default::default();
    env::doc_get_num32("aid\0", &mut aid);
    let mut key = KeyAccount {
        prefix: env::KeyPrefix {
            cid,
            ..Default::default()
        },
        key_in_contract: Key {
            account: Default::default(),
            aid,
        },
    };
    derive_my_pk(&mut key.key_in_contract.account, &cid);

    let mut amount: *const Amount = 0 as *const Amount;
    let mut size_val: u32 = Default::default();
    let mut proof: *const merkle::Node = 0 as *const merkle::Node;
    let proof_size: u32 = env::var_get_proof(
        &key,
        size_of_val(&key) as u32,
        &mut amount,
        &mut size_val,
        &mut proof,
    );
    if proof_size > 0 && size_of_val(&(unsafe { *amount })) as u32 == size_val {
        env::doc_add_num64("Amount\0", unsafe { *amount });
        env::doc_add_blob(
            "proof\0",
            proof,
            size_of_val(unsafe { &(*proof) }) as u32 * proof_size,
        );
    }
}

fn on_action_deposit(cid: ContractID) {
    let mut aid: AssetID = Default::default();
    let mut foreign_key = Default::default();
    let mut cosigner: u32 = Default::default();
    let mut amount: Amount = Default::default();
    env::doc_get_num32("aid\0", &mut aid);
    env::doc_get_blob(
        "foreign_key\0",
        &mut foreign_key,
        size_of_val(&foreign_key) as u32,
    );
    env::doc_get_num32("cosigner\0", &mut cosigner);
    env::doc_get_num64("amount\0", &mut amount);
    my_account_move(true, &cid, &foreign_key, cosigner, amount, aid);
}

fn on_action_withdraw(cid: ContractID) {
    let mut aid: AssetID = Default::default();
    let mut foreign_key = Default::default();
    let mut cosigner: u32 = Default::default();
    let mut amount: Amount = Default::default();
    env::doc_get_num32("aid\0", &mut aid);
    env::doc_get_blob(
        "foreign_key\0",
        &mut foreign_key,
        size_of_val(&foreign_key) as u32,
    );
    env::doc_get_num32("cosigner\0", &mut cosigner);
    env::doc_get_num64("amount\0", &mut amount);
    my_account_move(false, &cid, &foreign_key, cosigner, amount, aid);
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

    env::doc_add_group("view_account\0");
    env::doc_add_text("cid\0", "ContractID\0".as_ptr());
    env::doc_add_text("pubkey\0", "PubKey\0".as_ptr());
    env::doc_close_group(); // view_account

    env::doc_add_group("view_accounts\0");
    env::doc_add_text("cid\0", "ContractID\0".as_ptr());
    env::doc_close_group(); // view_accounts

    env::doc_add_group("view_logs\0");
    env::doc_add_text("cid\0", "ContractID\0".as_ptr());
    env::doc_close_group(); // view_logs

    env::doc_close_group(); // manager
    env::doc_add_group("my_account\0");

    env::doc_add_group("deposit\0");
    env::doc_add_text("cid\0", "ContractID\0".as_ptr());
    env::doc_add_text("amount\0", "Amount\0".as_ptr());
    env::doc_add_text("aid\0", "AssetID\0".as_ptr());
    env::doc_add_text("cosigner\0", "uint32_t\0".as_ptr());
    env::doc_add_text("foreign_key\0", "PubKey\0".as_ptr());
    env::doc_close_group(); // deposit

    env::doc_add_group("get_key\0");
    env::doc_add_text("cid\0", "ContractID\0".as_ptr());
    env::doc_close_group(); // get_key

    env::doc_add_group("get_proof\0");
    env::doc_add_text("cid\0", "ContractID\0".as_ptr());
    env::doc_add_text("aid\0", "AssetID\0".as_ptr());
    env::doc_close_group(); // get_proof

    env::doc_add_group("view\0");
    env::doc_add_text("cid\0", "ContractID\0".as_ptr());
    env::doc_close_group(); // view

    env::doc_add_group("withdraw\0");
    env::doc_add_text("cid\0", "ContractID\0".as_ptr());
    env::doc_add_text("amount\0", "Amount\0".as_ptr());
    env::doc_add_text("aid\0", "AssetID\0".as_ptr());
    env::doc_add_text("cosigner\0", "uint32_t\0".as_ptr());
    env::doc_add_text("foreign_key\0", "PubKey\0".as_ptr());
    env::doc_close_group(); // withdraw

    env::doc_close_group(); // my_account
    env::doc_close_group(); // roles
    env::doc_close_group(); // \0
}

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
