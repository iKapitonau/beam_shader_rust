#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    pub struct KeyTag {
    }

    impl KeyTag {
        pub const kInternal: u8 = 0;
        pub const kInternalStealth: u8 = 8;
        pub const kLockedAmount: u8 = 1;
        pub const kRefs: u8 = 2;
        pub const kOwnedAsset: u8 = 3;
        pub const kShaderChange: u8 = 4;
        pub const kSidCid: u8 = 16;
        pub const kMaxSize: u32 = 256;
    }

    pub mod Env {
        #[allow(unused_imports)]
        use self::super::super::root;

        pub fn SaveVar<K, V>(
                key: *const K,
                key_size: u32,
                val: *const V,
                val_size: u32,
                tag: u8,
        ) -> u32 {
            unsafe { return _SaveVar(key as *const u32, key_size, val as *const u32, val_size, tag); }
        }

        extern "C" {
            #[link_name = "SaveVar"]
            fn _SaveVar(
                pKey: *const u32,
                nKey: u32,
                pVal: *const u32,
                nVal: u32,
                nType: u8,
            ) -> u32;
        }
    }
}
