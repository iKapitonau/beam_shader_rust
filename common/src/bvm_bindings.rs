#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    pub type Height = u64;
    pub type Amount = u64;
    pub type AssetID = u32;
    pub type Timestamp = u64;
    pub type ContractID = [u8; 32usize];
    pub type ShaderID = [u8; 32usize];
    pub type HashValue = [u8; 32usize];
    pub type Secp_scalar_data = [u8; 32usize];

    #[repr(C)]
    pub struct Secp_point_data {
        pub X: [u8; 32usize],
        pub Y: u8,
    }

    pub type PubKey = Secp_point_data;

    #[repr(C)]
    pub struct FundsChange {
        pub m_Amount: Amount,
        pub m_Aid: AssetID,
        pub m_Consume: u8,
    }

    #[repr(C, packed)]
    pub struct SigRequest {
        pub m_pID: *const u32,
        pub m_nID: u32,
    }

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
        use crate::root::ContractID;

        #[repr(packed(1))]
        pub struct KeyPrefix {
            pub cid: ContractID,
            pub tag: u8,
        }

        #[repr(packed(1))]
        pub struct Key_T<T> {
            pub prefix: KeyPrefix,
            pub key_in_contract: T,
        }

        pub struct VarReaderEx<const flexible: bool> {
            handle: u32,
        }

        impl<const flexible: bool> VarReaderEx<flexible> {
            fn EnumInternal(&mut self, key1: *const u32, key1_size: u32, key2: *const u32, key2_size: u32) {
                self.handle = Vars_Enum(key1, key1_size, key2, key2_size); 
            }

            fn CloseInternal(&self) {
                if flexible {
                    if self.handle != 0 {
                        return;
                    }
                }
                Vars_Close(self.handle);
            }

            pub fn MoveNext(&self, key: *mut u32, key_size: &mut u32, val: *mut u32, val_size: &mut u32, repeat: u8) -> bool {
                Vars_MoveNext(self.handle, key, key_size, val, val_size, 0) != 0
            }

            pub fn Read<K, V>(key: *const K, key_size: u32, value: *mut V, val_size: u32) -> bool {
                let mut r = VarReaderEx::<false> {
                    handle: Default::default(),
                };
                r.EnumInternal(key as *const u32, key_size, key as *const u32, key_size);

                let mut key_size: u32 = 0;
                let mut val_size: u32 = val_size;
                let ret = r.MoveNext(0 as *mut u32, &mut key_size, value as *mut u32, &mut val_size, 0);
                r.CloseInternal();
                ret
            }
        }

        pub type VarReader = VarReaderEx<false>;

        pub fn SaveVar<K, V>(
                key: *const K,
                key_size: u32,
                val: *const V,
                val_size: u32,
                tag: u8,
        ) -> u32 {
            unsafe { return _SaveVar(key as *const u32, key_size, val as *const u32, val_size, tag); }
        }

        pub fn DocAddText<V>(id: &str, val: *const V) {
            unsafe { return _DocAddText(id.as_ptr() as *const u32, val as *const u32); }
        }

        pub fn DocGetText<V>(id: &str, val: *mut V, val_size: u32) -> u32 {
            unsafe { return _DocGetText(id.as_ptr() as *const u32, val as *mut u32, val_size); }
        }

        pub fn DocAddNum32(id: &str, val: u32) {
            unsafe { return _DocAddNum32(id.as_ptr() as *const u32, val); }
        }

        pub fn DocGetNum32(id: &str, out: *mut u32) -> u8 {
            unsafe { return _DocGetNum32(id.as_ptr() as *const u32, out); }
        }

        pub fn DocAddNum64(id: &str, val: u64) {
            unsafe { return _DocAddNum64(id.as_ptr() as *const u32, val); }
        }

        pub fn DocAddBlob<V>(id: &str, val: *const V, val_size: u32) {
            unsafe { return _DocAddBlob(id.as_ptr() as *const u32, val as *const u32, val_size); }
        }

        pub fn DocGetBlob<V>(id: &str, val: *mut V, val_size: u32) -> u32 {
            unsafe { return _DocGetBlob(id.as_ptr() as *const u32, val as *mut u32, val_size); }
        }

        pub fn DocAddGroup(id: &str) {
            unsafe { return _DocAddGroup(id.as_ptr() as *const u32); }
        }

        pub fn DocCloseGroup() {
            unsafe { return _DocCloseGroup(); }
        }

        pub fn DocAddArray(id: &str) {
            unsafe { return _DocAddArray(id.as_ptr() as *const u32); }
        }

        pub fn DocCloseArray() {
            unsafe { return _DocCloseArray(); }
        }

        pub fn Memset<V>(dst: *mut V, val: u8, size: u32) -> *mut u32 {
            unsafe { return _Memset(dst as *mut u32, val, size); }
        }

        pub fn Memcpy<S, D>(dst: *mut D, src: *mut S, size: u32) -> *mut u32 {
            unsafe { return _Memcpy(dst as *mut u32, src as *mut u32, size); }
        }

        pub fn Memcmp<S, D>(p1: *const S, p2: *const D, size: u32) -> i32 {
            unsafe { return _Memcmp(p1 as *const u32, p2 as *const u32, size); }
        }

        pub fn Strlen<V>(p: *const V) -> u32 {
            unsafe { return _Strlen(p as *const u32); }
        }

        pub fn Heap_Alloc(size: u32) -> *mut u32 {
            unsafe { return _Heap_Alloc(size); }
        }

        pub fn Heap_Free<V>(p: *mut V) {
            unsafe { return _Heap_Free(p as *mut u32); }
        }

        pub fn Vars_Close(slot: u32) {
            unsafe { return _Vars_Close(slot); }
        }

        pub fn Vars_Enum<U, V>(key0: *const U, key0_size: u32, key1: *const V, key1_size: u32) -> u32 {
            unsafe { return _Vars_Enum(key0 as *const u32, key0_size, key1 as *const u32, key1_size); }
        }

        pub fn Vars_MoveNext<K, V>(slot: u32, key: *mut K, key_size: *mut u32, val: *mut V, val_size: *mut u32, repeat: u8) -> u8 {
            unsafe { return _Vars_MoveNext(slot, key as *mut u32, key_size, val as *mut u32, val_size, repeat); }
        }

        pub fn GenerateKernel<U, V>(
            cid: *const root::ContractID,
            method: u32,
            arg: *const U,
            arg_size: u32,
            funds: *const root::FundsChange,
            funds_size: u32,
            sigs: *const root::SigRequest,
            sigs_size: u32,
            comment: *const V,
            charge: u32) {
            unsafe { return _GenerateKernel(cid, method, arg as *const u32, arg_size, funds, funds_size, sigs, sigs_size, comment as *const u32, charge); }
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

            #[link_name = "DocAddText"]
            pub fn _DocAddText(
                szID: *const u32,
                val: *const u32,
            );

            #[link_name = "DocGetText"]
            pub fn _DocGetText(
                szID: *const u32,
                val: *mut u32,
                val_size: u32,
            ) -> u32;

            #[link_name = "DocAddNum32"]
            pub fn _DocAddNum32(
                szID: *const u32,
                val: u32,
            );

            #[link_name = "DocGetNum32"]
            pub fn _DocGetNum32(
                szID: *const u32,
                pOut: *mut u32,
            ) -> u8;

            #[link_name = "DocAddNum64"]
            pub fn _DocAddNum64(
                szID: *const u32,
                val: u64,
            );

            #[link_name = "DocAddBlob"]
            pub fn _DocAddBlob(
                szID: *const u32,
                pBlob: *const u32,
                nBlob: u32,
            );

            #[link_name = "DocGetBlob"]
            pub fn _DocGetBlob(
                szID: *const u32,
                pBlob: *mut u32,
                nBlob: u32,
            ) -> u32;

            #[link_name = "DocAddGroup"]
            pub fn _DocAddGroup(
                szID: *const u32,
            );

            #[link_name = "DocCloseGroup"]
            pub fn _DocCloseGroup();

            #[link_name = "DocAddArray"]
            pub fn _DocAddArray(
                szID: *const u32,
            );

            #[link_name = "DocCloseArray"]
            pub fn _DocCloseArray();

            #[link_name = "Memset"]
            pub fn _Memset(
                pDst: *mut u32,
                val: u8,
                size: u32
            ) -> *mut u32;

            #[link_name = "Memcpy"]
            pub fn _Memcpy(
                pDst: *mut u32,
                pSrc: *mut u32,
                size: u32
            ) -> *mut u32;

            #[link_name = "Memcmp"]
            pub fn _Memcmp(
                p1: *const u32,
                p2: *const u32,
                size: u32
            ) -> i32;

            #[link_name = "Strlen"]
            pub fn _Strlen(
                p: *const u32,
            ) -> u32;

            #[link_name = "Heap_Alloc"]
            pub fn _Heap_Alloc(
                size: u32,
            ) -> *mut u32;

            #[link_name = "Heap_Free"]
            pub fn _Heap_Free(p: *mut u32);

            #[link_name = "Vars_Close"]
            pub fn _Vars_Close(iSlot: u32);

            #[link_name = "Vars_Enum"]
            pub fn _Vars_Enum(pKey0: *const u32, nKey0: u32, pKey1: *const u32, nKey1: u32) -> u32;

            #[link_name = "Vars_MoveNext"]
            pub fn _Vars_MoveNext(iSlot: u32, pKey: *mut u32, nKey: *mut u32, pVal: *mut u32, nVal: *mut u32, nRepeat: u8) -> u8;

            #[link_name = "GenerateKernel"]
            fn _GenerateKernel(
                cid: *const root::ContractID,
                method: u32,
                arg: *const u32,
                arg_size: u32,
                funds: *const root::FundsChange,
                funds_size: u32,
                sigs: *const root::SigRequest,
                sigs_size: u32,
                comment: *const u32,
                charge: u32
            );
        }
    }
}
