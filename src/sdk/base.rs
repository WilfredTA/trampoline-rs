// // Base types, operations on base types
// use ckb_sdk::{Address, AddressPayload, AddressType, CodeHashIndex};
// use ckb_types::packed::{Byte32, Script as PackedScript};
// use ckb_types::{H256, H160, core::ScriptHashType, prelude::*};
// use std::io::Read;
//
// type EthAddress = H160;
// pub struct Script {
//
// }
// #[derive(Hash, Eq, PartialEq, Clone, Debug)]
// pub struct PwAddress {
//     lock_script: PackedScript,
//     type_: ScriptHashType,
// }
//
// impl Script {
//     pub fn to_hash(&self) -> H256 {
//
//     }
// }
//
// impl PwAddress {
//
//     pub fn new(addr: EthAddress, code_hash: H256) -> Self {
//
//         let script_type = ScriptHashType::Type;
//         let mut pw_lock_type_script = PackedScript::new_builder()
//             .hash_type(script_type.into())
//             .code_hash(code_hash.pack())
//             .args(addr);
//         let script_hash = pw_lock_type_script.h
//
//
//     }
//     pub fn to_lock_script() -> Script {
//
//     }
//
//     pub fn generate() -> Address {
//
//     }
//
//
// }
//
