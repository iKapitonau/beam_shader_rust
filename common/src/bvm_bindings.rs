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
        pub m_pID: *const usize,
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
            fn EnumInternal(&mut self, key1: *const usize, key1_size: u32, key2: *const usize, key2_size: u32) {
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

            pub fn MoveNext(&self, key: *mut usize, key_size: &mut u32, val: *mut usize, val_size: &mut u32, repeat: u8) -> bool {
                Vars_MoveNext(self.handle, key, key_size, val, val_size, 0) != 0
            }

            pub fn Read<K, V>(key: *const K, key_size: u32, value: *mut V, val_size: u32) -> bool {
                let mut r = VarReaderEx::<false> {
                    handle: Default::default(),
                };
                r.EnumInternal(key as *const usize, key_size, key as *const usize, key_size);

                let mut key_size: u32 = 0;
                let mut val_size: u32 = val_size;
                let ret = r.MoveNext(0 as *mut usize, &mut key_size, value as *mut usize, &mut val_size, 0);
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
            unsafe { return _SaveVar(key as *const usize, key_size, val as *const usize, val_size, tag); }
        }

        pub fn DocAddText<V>(id: &str, val: *const V) {
            unsafe { return _DocAddText(id.as_ptr() as *const usize, val as *const usize); }
        }

        pub fn DocGetText<V>(id: &str, val: *mut V, val_size: u32) -> u32 {
            unsafe { return _DocGetText(id.as_ptr() as *const usize, val as *mut usize, val_size); }
        }

        pub fn DocAddNum32(id: &str, val: u32) {
            unsafe { return _DocAddNum32(id.as_ptr() as *const usize, val); }
        }

        pub fn DocGetNum32(id: &str, out: *mut u32) -> u8 {
            unsafe { return _DocGetNum32(id.as_ptr() as *const usize, out); }
        }

        pub fn DocAddNum64(id: &str, val: u64) {
            unsafe { return _DocAddNum64(id.as_ptr() as *const usize, val); }
        }

        pub fn DocAddBlob<V>(id: &str, val: *const V, val_size: u32) {
            unsafe { return _DocAddBlob(id.as_ptr() as *const usize, val as *const usize, val_size); }
        }

        pub fn DocGetBlob<V>(id: &str, val: *mut V, val_size: u32) -> u32 {
            unsafe { return _DocGetBlob(id.as_ptr() as *const usize, val as *mut usize, val_size); }
        }

        pub fn DocAddGroup(id: &str) {
            unsafe { return _DocAddGroup(id.as_ptr() as *const usize); }
        }

        pub fn DocCloseGroup() {
            unsafe { return _DocCloseGroup(); }
        }

        pub fn DocAddArray(id: &str) {
            unsafe { return _DocAddArray(id.as_ptr() as *const usize); }
        }

        pub fn DocCloseArray() {
            unsafe { return _DocCloseArray(); }
        }

        pub fn Memset<V>(dst: *mut V, val: u8, size: u32) -> *mut usize {
            unsafe { return _Memset(dst as *mut usize, val, size); }
        }

        pub fn Memcpy<S, D>(dst: *mut D, src: *mut S, size: u32) -> *mut usize {
            unsafe { return _Memcpy(dst as *mut usize, src as *mut usize, size); }
        }

        pub fn Memcmp<S, D>(p1: *const S, p2: *const D, size: u32) -> i32 {
            unsafe { return _Memcmp(p1 as *const usize, p2 as *const usize, size); }
        }

        pub fn Strlen<V>(p: *const V) -> u32 {
            unsafe { return _Strlen(p as *const usize); }
        }

        pub fn Heap_Alloc(size: u32) -> *mut usize {
            unsafe { return _Heap_Alloc(size); }
        }

        pub fn Heap_Free<V>(p: *mut V) {
            unsafe { return _Heap_Free(p as *mut usize); }
        }

        pub fn Vars_Close(slot: u32) {
            unsafe { return _Vars_Close(slot); }
        }

        pub fn Vars_Enum<U, V>(key0: *const U, key0_size: u32, key1: *const V, key1_size: u32) -> u32 {
            unsafe { return _Vars_Enum(key0 as *const usize, key0_size, key1 as *const usize, key1_size); }
        }

        pub fn Vars_MoveNext<K, V>(slot: u32, key: *mut K, key_size: *mut u32, val: *mut V, val_size: *mut u32, repeat: u8) -> u8 {
            unsafe { return _Vars_MoveNext(slot, key as *mut usize, key_size, val as *mut usize, val_size, repeat); }
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
            unsafe { return _GenerateKernel(cid, method, arg as *const usize, arg_size, funds, funds_size, sigs, sigs_size, comment as *const usize, charge); }
        }

        extern "C" {
            #[link_name = "SaveVar"]
            fn _SaveVar(
                pKey: *const usize,
                nKey: u32,
                pVal: *const usize,
                nVal: u32,
                nType: u8,
            ) -> u32;

            #[link_name = "DocAddText"]
            pub fn _DocAddText(
                szID: *const usize,
                val: *const usize,
            );

            #[link_name = "DocGetText"]
            pub fn _DocGetText(
                szID: *const usize,
                val: *mut usize,
                val_size: u32,
            ) -> u32;

            #[link_name = "DocAddNum32"]
            pub fn _DocAddNum32(
                szID: *const usize,
                val: u32,
            );

            #[link_name = "DocGetNum32"]
            pub fn _DocGetNum32(
                szID: *const usize,
                pOut: *mut u32,
            ) -> u8;

            #[link_name = "DocAddNum64"]
            pub fn _DocAddNum64(
                szID: *const usize,
                val: u64,
            );

            #[link_name = "DocAddBlob"]
            pub fn _DocAddBlob(
                szID: *const usize,
                pBlob: *const usize,
                nBlob: u32,
            );

            #[link_name = "DocGetBlob"]
            pub fn _DocGetBlob(
                szID: *const usize,
                pBlob: *mut usize,
                nBlob: u32,
            ) -> u32;

            #[link_name = "DocAddGroup"]
            pub fn _DocAddGroup(
                szID: *const usize,
            );

            #[link_name = "DocCloseGroup"]
            pub fn _DocCloseGroup();

            #[link_name = "DocAddArray"]
            pub fn _DocAddArray(
                szID: *const usize,
            );

            #[link_name = "DocCloseArray"]
            pub fn _DocCloseArray();

            #[link_name = "Memset"]
            pub fn _Memset(
                pDst: *mut usize,
                val: u8,
                size: u32
            ) -> *mut usize;

            #[link_name = "Memcpy"]
            pub fn _Memcpy(
                pDst: *mut usize,
                pSrc: *mut usize,
                size: u32
            ) -> *mut usize;

            #[link_name = "Memcmp"]
            pub fn _Memcmp(
                p1: *const usize,
                p2: *const usize,
                size: u32
            ) -> i32;

            #[link_name = "Strlen"]
            pub fn _Strlen(
                p: *const usize,
            ) -> u32;

            #[link_name = "Heap_Alloc"]
            pub fn _Heap_Alloc(
                size: u32,
            ) -> *mut usize;

            #[link_name = "Heap_Free"]
            pub fn _Heap_Free(p: *mut usize);

            #[link_name = "Vars_Close"]
            pub fn _Vars_Close(iSlot: u32);

            #[link_name = "Vars_Enum"]
            pub fn _Vars_Enum(pKey0: *const usize, nKey0: u32, pKey1: *const usize, nKey1: u32) -> u32;

            #[link_name = "Vars_MoveNext"]
            pub fn _Vars_MoveNext(iSlot: u32, pKey: *mut usize, nKey: *mut u32, pVal: *mut usize, nVal: *mut u32, nRepeat: u8) -> u8;

            #[link_name = "GenerateKernel"]
            fn _GenerateKernel(
                cid: *const root::ContractID,
                method: u32,
                arg: *const usize,
                arg_size: u32,
                funds: *const root::FundsChange,
                funds_size: u32,
                sigs: *const root::SigRequest,
                sigs_size: u32,
                comment: *const usize,
                charge: u32
            );
        }
    }
}
