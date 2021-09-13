use crate::rpc::*;
use crate::rpc::{get_pw_tx_info, get_sudt_tx_info};
use crate::DEV_RPC_URL;
use anyhow::Result;
use ckb_sdk::rpc::{
    CellDep, DepType, JsonBytes, OutPoint, Script, ScriptHashType, TransactionWithStatus,
};
use ckb_types::bytes::Bytes;
use ckb_types::core;
use ckb_types::packed::{Uint32, Uint32Builder};
use ckb_types::prelude::{Entity, Pack, PackVec};
use ckb_types::H256;
use serde::{Deserialize, Serialize};
use serde_json;
use std::borrow::Borrow;
use std::convert::TryFrom;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use toml;

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwScriptRef {
    cell_dep: CellDep,
    script: Script,
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwConfig {
    dao_type: PwScriptRef,
    default_lock: PwScriptRef,
    pw_lock: PwScriptRef,
    sudt_type: PwScriptRef,
    multi_sig_lock: PwScriptRef,
    acp_lock_list: Vec<Script>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChainConfig {
    pub ckb_dev: DevConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DevConfig {
    pub spec_hash: String,
    pub genesis: String,
    pub cellbase: String,
    pub dep_groups: Vec<DepGroupConfig>,
    pub system_cells: Vec<SysCellConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SysCellConfig {
    pub path: String,
    pub tx_hash: String,
    pub index: u32,
    pub data_hash: String,
    pub type_hash: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DepGroupConfig {
    pub included_cells: Vec<String>,
    pub tx_hash: String,
    pub index: u32,
}

pub fn read_hash_toml() -> Result<ChainConfig> {
    let toml_ = fs::read_to_string("./ckb-hashes.toml")?;
    let decoded: ChainConfig = toml::from_str(toml_.as_str())?;
    let as_json = serde_json::to_string(&decoded)?;
    let as_json = as_json.as_str();
    fs::write("./ckb-hashes.json", as_json)?;
    Ok(decoded)
}

#[derive(Debug, Deserialize, Serialize)]
struct RawOutpoint {
    pub tx_hash: String,
    pub code_hash: String,
    pub index: u32,
}

enum SysCellSelected {
    DaoType,
    DefaultLock,
    PwLock,
    Sudt,
}

// To do: Remove using ckb_sdk types and take advantage of serialization implementations.
// ckb_sdk types do not serialize exactly as required
pub fn gen_config() -> Result<()> {
    let hashes_json = fs::read_to_string("./ckb-hashes.json")?;
    let chain_config: ChainConfig = serde_json::from_str(hashes_json.as_str())?;
    let sys_cells = &chain_config.ckb_dev.system_cells;
    gen_syscell_config(sys_cells, SysCellSelected::DaoType)?;
    gen_syscell_config(sys_cells, SysCellSelected::DefaultLock)?;
    gen_syscell_config(sys_cells, SysCellSelected::Sudt)?;
    Ok(())
}

fn gen_syscell_config(
    sys_cells: &Vec<SysCellConfig>,
    type_: SysCellSelected,
) -> Result<PwScriptRef> {
    return match type_ {
        SysCellSelected::DaoType => gen_dao_config(sys_cells),
        SysCellSelected::DefaultLock => gen_default_lock_config(sys_cells),
        SysCellSelected::Sudt => gen_sudt_config(),
        _ => gen_default_lock_config(sys_cells),
    };
}

fn gen_dao_config(sys_cells: &Vec<SysCellConfig>) -> Result<PwScriptRef> {
    let mut raw_dao_out = RawOutpoint {
        tx_hash: sys_cells[1].tx_hash.clone(),
        code_hash: sys_cells[1].type_hash.as_ref().unwrap().clone(),
        index: sys_cells[1].index.clone(),
    };

    let dao_script = build_script(&raw_dao_out.code_hash, "type", "0x")?;
    let dao_cell_dep = build_cell_dep(&raw_dao_out.tx_hash, raw_dao_out.index, "code")?;

    let dao_pw_obj = PwScriptRef {
        script: dao_script,
        cell_dep: dao_cell_dep,
    };
    fs::write("./pw-config-dao.json", serde_json::to_string(&dao_pw_obj)?)?;

    Ok(dao_pw_obj)
}

fn gen_default_lock_config(sys_cells: &Vec<SysCellConfig>) -> Result<PwScriptRef> {
    let mut raw_lock_out = RawOutpoint {
        tx_hash: sys_cells[0].tx_hash.clone(),
        code_hash: sys_cells[0].type_hash.as_ref().unwrap().clone(),
        index: sys_cells[0].index.clone(),
    };

    let lock_script = build_script(&raw_lock_out.code_hash, "type", "0x")?;
    let lock_dep = build_cell_dep(&raw_lock_out.tx_hash, raw_lock_out.index, "code")?;

    let lock_pw_obj = PwScriptRef {
        script: lock_script,
        cell_dep: lock_dep,
    };

    fs::write(
        "./pw-config-default-lock.json",
        serde_json::to_string(&lock_pw_obj)?,
    )?;

    Ok(lock_pw_obj)
}

fn gen_sudt_config() -> Result<PwScriptRef> {
    let sudt_info = get_sudt_tx_info(DEV_RPC_URL)?;
    let tx_hash = sudt_info.transaction.hash;
    let index: u32 = 0;
    let code_hash = sudt_info.transaction.inner.outputs[0]
        .type_
        .as_ref()
        .unwrap()
        .code_hash
        .clone();
    let args = "0x";

    let args = hex::decode(args.trim_start_matches("0x"))?;
    let args = Bytes::copy_from_slice(args.as_slice());
    println!("Args as bytes: {:?}", args);
    let args = JsonBytes::from_bytes(args);

    let script = Script {
        code_hash,
        hash_type: ScriptHashType::Type,
        args,
    };

    let out_point = OutPoint { tx_hash, index };

    let cell_dep = CellDep {
        out_point,
        dep_type: DepType::Code,
    };

    let sudt_pw_obj = PwScriptRef { script, cell_dep };

    fs::write(
        "./pw-config-sudt.json",
        serde_json::to_string(&sudt_pw_obj)?,
    )?;
    Ok(sudt_pw_obj)
}
fn build_cell_dep(tx_hash: &str, index: u32, dep_type: &str) -> Result<CellDep> {
    let tx_hash = tx_hash.trim_start_matches("0x");
    let dep_type = match dep_type {
        "code" => DepType::Code,
        "group" => DepType::DepGroup,
        _ => DepType::Code,
    };

    let out_point = OutPoint {
        tx_hash: H256::from_str(tx_hash)?,
        index,
    };

    Ok(CellDep {
        out_point,
        dep_type,
    })
}
fn build_script(code_hash: &str, hash_type: &str, args: &str) -> Result<Script> {
    let code_hash = code_hash.trim_start_matches("0x");
    let code_hash = H256::from_str(code_hash)?;

    let script_hash_type = match hash_type {
        "type" => ScriptHashType::Type,
        "data" => ScriptHashType::Data,
        _ => {
            panic!("Invalid hash type");
        }
    };

    let args = hex::decode(args.trim_start_matches("0x"))?;
    let args = Bytes::copy_from_slice(args.as_slice());
    println!("Args as bytes: {:?}", args);
    let args = JsonBytes::from_bytes(args);
    println!("Args as json bytes: {:?}", args);

    Ok(Script {
        code_hash,
        hash_type: script_hash_type,
        args,
    })
}
