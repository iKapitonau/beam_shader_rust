pub mod common {
    pub type Height = u64;
    pub type Amount = u64;
    pub type AssetID = u32;
    pub type Timestamp = u64;
    pub type ContractID = [u8; 32usize];
    pub type ShaderID = [u8; 32usize];
    pub type HashValue = [u8; 32usize];
    pub type SecpScalarData = [u8; 32usize];

    pub mod merkle {
        #[repr(C)]
        pub struct Node {
            first: bool,
            second: [u8; 32],
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SecpPointData {
        pub x: [u8; 32usize],
        pub y: u8,
    }

    pub type PubKey = SecpPointData;

    #[repr(C)]
    pub struct FundsChange {
        pub amount: Amount,
        pub aid: AssetID,
        pub consume: u8,
    }

    #[repr(C, packed)]
    pub struct SigRequest {
        pub id_ptr: *const usize,
        pub id_size: u32,
    }

    impl SigRequest {
        pub fn get_pk(&self, pk: &mut PubKey) {
            let id_ptr: *const usize = self.id_ptr;
            env::derive_pk(pk, &id_ptr, self.id_size);
        }
    }

    pub type KeyID = SigRequest;

    #[repr(C)]
    pub struct HeightPos {
        pub height: Height,
        pub pos: u32,
    }

    pub struct KeyTag {}

    impl KeyTag {
        pub const INTERNAL: u8 = 0;
        pub const INTERNAL_STEALTH: u8 = 8;
        pub const LOCKED_AMOUNT: u8 = 1;
        pub const REFS: u8 = 2;
        pub const OWNED_ASSET: u8 = 3;
        pub const SHADER_CHANGE: u8 = 4;
        pub const SID_CID: u8 = 16;
        pub const MAX_SIZE: u32 = 256;
    }

    pub mod env {
        use crate::common::*;
        use core::mem::size_of_val;

        #[repr(C, packed(1))]
        pub struct KeyPrefix {
            pub cid: ContractID,
            pub tag: u8,
        }

        #[repr(C, packed(1))]
        pub struct Key<T> {
            pub prefix: KeyPrefix,
            pub key_in_contract: T,
        }

        pub struct VarReaderEx<const FLEXIBLE: bool> {
            handle: u32,
        }

        impl<const FLEXIBLE: bool> VarReaderEx<FLEXIBLE> {
            fn enum_internal(
                &mut self,
                key1: *const usize,
                key1_size: u32,
                key2: *const usize,
                key2_size: u32,
            ) {
                self.handle = vars_enum(key1, key1_size, key2, key2_size);
            }

            fn close_internal(&self) {
                if FLEXIBLE {
                    if self.handle == 0 {
                        return;
                    }
                }
                vars_close(self.handle);
            }

            pub fn new(
                key1: *const usize,
                key1_size: u32,
                key2: *const usize,
                key2_size: u32,
            ) -> VarReader {
                let mut r = VarReader {
                    handle: Default::default(),
                };
                r.enum_internal(key1, key1_size, key2, key2_size);
                r
            }

            pub fn move_next(
                &self,
                key: *mut usize,
                key_size: &mut u32,
                val: *mut usize,
                val_size: &mut u32,
                repeat: u8,
            ) -> bool {
                vars_move_next(self.handle, key, key_size, val, val_size, repeat) != 0
            }

            pub fn move_next_t<K, V>(&self, key: &mut K, value: &mut V) -> bool {
                loop {
                    let mut key_size: u32 = size_of_val(key) as u32;
                    let mut value_size: u32 = size_of_val(value) as u32;
                    if !self.move_next(
                        key as *mut K as *mut usize,
                        &mut key_size,
                        value as *mut V as *mut usize,
                        &mut value_size,
                        0,
                    ) {
                        return false;
                    }
                    if size_of_val(key) as u32 == key_size
                        && size_of_val(value) as u32 == value_size
                    {
                        break;
                    }
                }
                true
            }

            pub fn read<K, V>(key: &K, value: &mut V) -> bool {
                let mut r = VarReaderEx::<false> {
                    handle: Default::default(),
                };

                let mut key_size: u32 = size_of_val(key) as u32;
                r.enum_internal(
                    key as *const K as *const usize,
                    key_size,
                    key as *const K as *const usize,
                    key_size,
                );

                let mut val_size: u32 = size_of_val(value) as u32;
                key_size = 0;
                let ret = r.move_next(
                    0 as *mut usize,
                    &mut key_size,
                    value as *mut V as *mut usize,
                    &mut val_size,
                    0,
                ) && size_of_val(value) as u32 == val_size;
                r.close_internal();
                ret
            }

            pub fn r#enum<K, V>(&mut self, key: &K, value: &V) {
                self.close_internal();
                let key_size: u32 = size_of_val(key) as u32;
                let value_size: u32 = size_of_val(value) as u32;
                self.enum_internal(
                    key as *const K as *const usize,
                    key_size,
                    value as *const V as *const usize,
                    value_size,
                );
            }
        }

        pub type VarReader = VarReaderEx<false>;

        pub struct LogReader {
            handle: u32,
            pub pos: HeightPos,
        }

        impl LogReader {
            pub fn new(
                key1: *const usize,
                key1_size: u32,
                key2: *const usize,
                key2_size: u32,
                pos_min: *const HeightPos,
                pos_max: *const HeightPos,
            ) -> LogReader {
                LogReader {
                    handle: logs_enum(key1, key1_size, key2, key2_size, pos_min, pos_max),
                    pos: HeightPos {
                        height: Default::default(),
                        pos: Default::default(),
                    },
                }
            }

            pub fn move_next(
                &mut self,
                key: *mut usize,
                key_size: &mut u32,
                val: *mut usize,
                val_size: &mut u32,
                repeat: u8,
            ) -> bool {
                logs_move_next(
                    self.handle,
                    key,
                    key_size,
                    val,
                    val_size,
                    &mut self.pos,
                    repeat,
                ) != 0
            }

            pub fn move_next_t<K, V>(&mut self, key: &mut K, value: &mut V) -> bool {
                loop {
                    let mut key_size: u32 = size_of_val(key) as u32;
                    let mut value_size: u32 = size_of_val(value) as u32;
                    if !self.move_next(
                        key as *mut K as *mut usize,
                        &mut key_size,
                        value as *mut V as *mut usize,
                        &mut value_size,
                        0,
                    ) {
                        return false;
                    }
                    if size_of_val(key) as u32 == key_size
                        && size_of_val(value) as u32 == value_size
                    {
                        break;
                    }
                }
                true
            }
        }

        impl Drop for LogReader {
            fn drop(&mut self) {
                logs_close(self.handle);
            }
        }

        #[repr(C, packed(1))]
        struct SidCid {
            sid: ShaderID,
            cid: ContractID,
        }

        type KeySidCid = Key<SidCid>;

        struct ContractsWalker {
            pub key: KeySidCid,
            pub height: Height,
            pub reader: VarReaderEx<true>,
        }

        impl ContractsWalker {
            pub fn r#enum(&mut self, sid: &ShaderID) {
                let k0 = KeySidCid {
                    prefix: KeyPrefix {
                        cid: Default::default(),
                        tag: KeyTag::SID_CID,
                    },
                    key_in_contract: SidCid {
                        cid: Default::default(),
                        sid: *sid,
                    },
                };
                let k1 = KeySidCid {
                    prefix: KeyPrefix { ..k0.prefix },
                    key_in_contract: SidCid {
                        cid: [0xff; 32],
                        ..k0.key_in_contract
                    },
                };
                self.reader.r#enum(&k0, &k1);
            }

            pub fn move_next(&mut self) -> bool {
                if !self.reader.move_next_t(&mut self.key, &mut self.height) {
                    return false;
                }
                self.height = u64::from_be(self.height);
                return true;
            }
        }

        pub fn enum_and_dump_contracts(sid: &ShaderID) {
            doc_add_array("contracts\0");

            let mut wlk = ContractsWalker {
                key: KeySidCid {
                    prefix: KeyPrefix {
                        cid: [0; 32],
                        tag: 0,
                    },
                    key_in_contract: SidCid {
                        cid: [0; 32],
                        sid: [1; 32],
                    },
                },
                height: 0,
                reader: VarReaderEx::<true> { handle: 0 },
            };
            wlk.r#enum(&sid);
            while wlk.move_next() {
                doc_add_group("\0");
                doc_add_blob(
                    "cid\0",
                    &wlk.key.key_in_contract.cid,
                    size_of_val(&wlk.key.key_in_contract.cid) as u32,
                );
                doc_add_num64("height\0", wlk.height);
                doc_close_group();
            }
            doc_close_array();
        }

        pub fn var_get_proof<K, V>(
            key: *const K,
            key_size: u32,
            val: *mut *const V,
            val_size: *mut u32,
            proof: *mut *const merkle::Node,
        ) -> u32 {
            unsafe {
                return _VarGetProof(
                    key as *const usize,
                    key_size,
                    val as *mut *const usize,
                    val_size,
                    proof,
                );
            }
        }

        pub fn derive_pk<T>(pubkey: &mut PubKey, id: *const T, id_size: u32) {
            unsafe {
                return _DerivePk(pubkey, id as *const usize, id_size);
            }
        }

        pub fn funds_lock(aid: AssetID, amount: Amount) {
            unsafe {
                return _FundsLock(aid, amount);
            }
        }

        pub fn funds_unlock(aid: AssetID, amount: Amount) {
            unsafe {
                return _FundsUnlock(aid, amount);
            }
        }

        pub fn add_sig(pubkey: &PubKey) {
            unsafe {
                return _AddSig(pubkey);
            }
        }

        pub fn halt_if(condition: bool) {
            unsafe {
                if condition {
                    return _Halt();
                }
            }
        }

        pub fn halt() {
            unsafe {
                return _Halt();
            }
        }

        pub fn emit_log<K, V>(
            key: *const K,
            key_size: u32,
            value: *const V,
            value_size: u32,
            tag: u8,
        ) -> u32 {
            unsafe {
                return _EmitLog(
                    key as *const usize,
                    key_size,
                    value as *const usize,
                    value_size,
                    tag,
                );
            }
        }

        pub fn load_var<K, V>(
            key: *const K,
            key_size: u32,
            value: *const V,
            value_size: u32,
            tag: u8,
        ) -> u32 {
            unsafe {
                return _LoadVar(
                    key as *const usize,
                    key_size,
                    value as *const usize,
                    value_size,
                    tag,
                );
            }
        }

        pub fn del_var<K>(key: *const K, key_size: u32) -> u32 {
            unsafe {
                return _SaveVar(
                    key as *const usize,
                    key_size,
                    0 as *const usize,
                    0,
                    KeyTag::INTERNAL,
                );
            }
        }

        pub fn save_var<K, V>(
            key: *const K,
            key_size: u32,
            val: *const V,
            val_size: u32,
            tag: u8,
        ) -> u32 {
            unsafe {
                return _SaveVar(
                    key as *const usize,
                    key_size,
                    val as *const usize,
                    val_size,
                    tag,
                );
            }
        }

        pub fn doc_add_text<V>(id: &str, val: *const V) {
            unsafe {
                return _DocAddText(id.as_ptr() as *const usize, val as *const usize);
            }
        }

        pub fn doc_get_text<V>(id: &str, val: *mut V, val_size: u32) -> u32 {
            unsafe {
                return _DocGetText(id.as_ptr() as *const usize, val as *mut usize, val_size);
            }
        }

        pub fn doc_add_num32(id: &str, val: u32) {
            unsafe {
                return _DocAddNum32(id.as_ptr() as *const usize, val);
            }
        }

        pub fn doc_get_num64(id: &str, out: *mut u64) -> u8 {
            unsafe {
                return _DocGetNum64(id.as_ptr() as *const usize, out);
            }
        }

        pub fn doc_get_num32(id: &str, out: *mut u32) -> u8 {
            unsafe {
                return _DocGetNum32(id.as_ptr() as *const usize, out);
            }
        }

        pub fn doc_add_num64(id: &str, val: u64) {
            unsafe {
                return _DocAddNum64(id.as_ptr() as *const usize, val);
            }
        }

        pub fn doc_add_blob<V>(id: &str, val: *const V, val_size: u32) {
            unsafe {
                return _DocAddBlob(id.as_ptr() as *const usize, val as *const usize, val_size);
            }
        }

        pub fn doc_get_blob<V>(id: &str, val: *mut V, val_size: u32) -> u32 {
            unsafe {
                return _DocGetBlob(id.as_ptr() as *const usize, val as *mut usize, val_size);
            }
        }

        pub fn doc_add_group(id: &str) {
            unsafe {
                return _DocAddGroup(id.as_ptr() as *const usize);
            }
        }

        pub fn doc_close_group() {
            unsafe {
                return _DocCloseGroup();
            }
        }

        pub fn doc_add_array(id: &str) {
            unsafe {
                return _DocAddArray(id.as_ptr() as *const usize);
            }
        }

        pub fn doc_close_array() {
            unsafe {
                return _DocCloseArray();
            }
        }

        pub fn mem_is_0<T>(ptr: *const T, size: u32) -> u8 {
            unsafe {
                return _Memis0(ptr as *const usize, size);
            }
        }

        pub fn memset<V>(dst: *mut V, val: u8, size: u32) -> *mut usize {
            unsafe {
                return _Memset(dst as *mut usize, val, size);
            }
        }

        pub fn memcpy<S, D>(dst: *mut D, src: *mut S, size: u32) -> *mut usize {
            unsafe {
                return _Memcpy(dst as *mut usize, src as *mut usize, size);
            }
        }

        pub fn memcmp<S, D>(p1: *const S, p2: *const D, size: u32) -> i32 {
            unsafe {
                return _Memcmp(p1 as *const usize, p2 as *const usize, size);
            }
        }

        pub fn strlen<V>(p: *const V) -> u32 {
            unsafe {
                return _Strlen(p as *const usize);
            }
        }

        pub fn heap_alloc(size: u32) -> *mut usize {
            unsafe {
                return _Heap_Alloc(size);
            }
        }

        pub fn heap_free<V>(p: *mut V) {
            unsafe {
                return _Heap_Free(p as *mut usize);
            }
        }

        pub fn logs_close(slot: u32) {
            unsafe {
                return _Logs_Close(slot);
            }
        }

        pub fn logs_enum<U, V>(
            key0: *const U,
            key0_size: u32,
            key1: *const V,
            key1_size: u32,
            pos_min: *const HeightPos,
            pos_max: *const HeightPos,
        ) -> u32 {
            unsafe {
                return _Logs_Enum(
                    key0 as *const usize,
                    key0_size,
                    key1 as *const usize,
                    key1_size,
                    pos_min,
                    pos_max,
                );
            }
        }

        pub fn logs_move_next<K, V>(
            slot: u32,
            key: *mut K,
            key_size: *mut u32,
            val: *mut V,
            val_size: *mut u32,
            pos: *const HeightPos,
            repeat: u8,
        ) -> u8 {
            unsafe {
                return _Logs_MoveNext(
                    slot,
                    key as *mut usize,
                    key_size,
                    val as *mut usize,
                    val_size,
                    pos,
                    repeat,
                );
            }
        }

        pub fn vars_close(slot: u32) {
            unsafe {
                return _Vars_Close(slot);
            }
        }

        pub fn vars_enum<U, V>(
            key0: *const U,
            key0_size: u32,
            key1: *const V,
            key1_size: u32,
        ) -> u32 {
            unsafe {
                return _Vars_Enum(
                    key0 as *const usize,
                    key0_size,
                    key1 as *const usize,
                    key1_size,
                );
            }
        }

        pub fn vars_move_next<K, V>(
            slot: u32,
            key: *mut K,
            key_size: *mut u32,
            val: *mut V,
            val_size: *mut u32,
            repeat: u8,
        ) -> u8 {
            unsafe {
                return _Vars_MoveNext(
                    slot,
                    key as *mut usize,
                    key_size,
                    val as *mut usize,
                    val_size,
                    repeat,
                );
            }
        }

        pub fn generate_kernel<U, V>(
            cid: *const ContractID,
            method: u32,
            arg: *const U,
            arg_size: u32,
            funds: *const FundsChange,
            funds_size: u32,
            sigs: *const SigRequest,
            sigs_size: u32,
            comment: *const V,
            charge: u32,
        ) {
            unsafe {
                return _GenerateKernel(
                    cid,
                    method,
                    arg as *const usize,
                    arg_size,
                    funds,
                    funds_size,
                    sigs,
                    sigs_size,
                    comment as *const usize,
                    charge,
                );
            }
        }

        extern "C" {
            #[link_name = "VarGetProof"]
            fn _VarGetProof(
                pKey: *const usize,
                nKey: u32,
                ppVal: *mut *const usize,
                pnVal: *mut u32,
                ppProof: *mut *const merkle::Node,
            ) -> u32;

            #[link_name = "DerivePk"]
            fn _DerivePk(pubkey: *mut PubKey, pID: *const usize, nID: u32);

            #[link_name = "FundsLock"]
            fn _FundsLock(aid: AssetID, amount: Amount);

            #[link_name = "FundsUnlock"]
            fn _FundsUnlock(aid: AssetID, amount: Amount);

            #[link_name = "AddSig"]
            fn _AddSig(pubkey: *const PubKey);

            #[link_name = "Halt"]
            fn _Halt();

            #[link_name = "EmitLog"]
            fn _EmitLog(
                pKey: *const usize,
                nKey: u32,
                pVal: *const usize,
                nVal: u32,
                nType: u8,
            ) -> u32;

            #[link_name = "LoadVar"]
            fn _LoadVar(
                pKey: *const usize,
                nKey: u32,
                pVal: *const usize,
                nVal: u32,
                nType: u8,
            ) -> u32;

            #[link_name = "SaveVar"]
            fn _SaveVar(
                pKey: *const usize,
                nKey: u32,
                pVal: *const usize,
                nVal: u32,
                nType: u8,
            ) -> u32;

            #[link_name = "DocAddText"]
            pub fn _DocAddText(szID: *const usize, val: *const usize);

            #[link_name = "DocGetText"]
            pub fn _DocGetText(szID: *const usize, val: *mut usize, val_size: u32) -> u32;

            #[link_name = "DocAddNum32"]
            pub fn _DocAddNum32(szID: *const usize, val: u32);

            #[link_name = "DocGetNum32"]
            pub fn _DocGetNum32(szID: *const usize, pOut: *mut u32) -> u8;

            #[link_name = "DocGetNum64"]
            pub fn _DocGetNum64(szID: *const usize, pOut: *mut u64) -> u8;

            #[link_name = "DocAddNum64"]
            pub fn _DocAddNum64(szID: *const usize, val: u64);

            #[link_name = "DocAddBlob"]
            pub fn _DocAddBlob(szID: *const usize, pBlob: *const usize, nBlob: u32);

            #[link_name = "DocGetBlob"]
            pub fn _DocGetBlob(szID: *const usize, pBlob: *mut usize, nBlob: u32) -> u32;

            #[link_name = "DocAddGroup"]
            pub fn _DocAddGroup(szID: *const usize);

            #[link_name = "DocCloseGroup"]
            pub fn _DocCloseGroup();

            #[link_name = "DocAddArray"]
            pub fn _DocAddArray(szID: *const usize);

            #[link_name = "DocCloseArray"]
            pub fn _DocCloseArray();

            #[link_name = "Memis0"]
            pub fn _Memis0(p: *const usize, sz: u32) -> u8;

            #[link_name = "Memset"]
            pub fn _Memset(pDst: *mut usize, val: u8, size: u32) -> *mut usize;

            #[link_name = "Memcpy"]
            pub fn _Memcpy(pDst: *mut usize, pSrc: *mut usize, size: u32) -> *mut usize;

            #[link_name = "Memcmp"]
            pub fn _Memcmp(p1: *const usize, p2: *const usize, size: u32) -> i32;

            #[link_name = "Strlen"]
            pub fn _Strlen(p: *const usize) -> u32;

            #[link_name = "Heap_Alloc"]
            pub fn _Heap_Alloc(size: u32) -> *mut usize;

            #[link_name = "Heap_Free"]
            pub fn _Heap_Free(p: *mut usize);

            #[link_name = "Vars_Close"]
            pub fn _Vars_Close(iSlot: u32);

            #[link_name = "Vars_Enum"]
            pub fn _Vars_Enum(
                pKey0: *const usize,
                nKey0: u32,
                pKey1: *const usize,
                nKey1: u32,
            ) -> u32;

            #[link_name = "Vars_MoveNext"]
            pub fn _Vars_MoveNext(
                iSlot: u32,
                pKey: *mut usize,
                nKey: *mut u32,
                pVal: *mut usize,
                nVal: *mut u32,
                nRepeat: u8,
            ) -> u8;

            #[link_name = "Logs_Close"]
            pub fn _Logs_Close(iSlot: u32);

            #[link_name = "Logs_Enum"]
            pub fn _Logs_Enum(
                pKey0: *const usize,
                nKey0: u32,
                pKey1: *const usize,
                nKey1: u32,
                pos_min: *const HeightPos,
                pos_max: *const HeightPos,
            ) -> u32;

            #[link_name = "Logs_MoveNext"]
            pub fn _Logs_MoveNext(
                iSlot: u32,
                pKey: *mut usize,
                nKey: *mut u32,
                pVal: *mut usize,
                nVal: *mut u32,
                pos: *const HeightPos,
                nRepeat: u8,
            ) -> u8;

            #[link_name = "GenerateKernel"]
            fn _GenerateKernel(
                cid: *const ContractID,
                method: u32,
                arg: *const usize,
                arg_size: u32,
                funds: *const FundsChange,
                funds_size: u32,
                sigs: *const SigRequest,
                sigs_size: u32,
                comment: *const usize,
                charge: u32,
            );
        }
    }
}
