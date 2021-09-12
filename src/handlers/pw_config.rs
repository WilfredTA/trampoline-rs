use crate::rpc::*;
use anyhow::Result;
use ckb_sdk::rpc::{CellDep, OutPoint, Script, ScriptHashType, TransactionWithStatus};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::Path;
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
    ckb_dev: DevConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DevConfig {
    spec_hash: String,
    genesis: String,
    cellbase: String,
    dep_groups: Vec<DepGroupConfig>,
    system_cells: Vec<SysCellConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SysCellConfig {
    path: String,
    tx_hash: String,
    index: u64,
    data_hash: String,
    type_hash: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DepGroupConfig {
    included_cells: Vec<String>,
    tx_hash: String,
    index: u64,
}

pub fn read_hash_toml() -> Result<ChainConfig> {
    let toml_ = fs::read_to_string("./ckb-hashes.toml")?;
    let decoded: ChainConfig = toml::from_str(toml_.as_str())?;
    let as_json = serde_json::to_string(&decoded)?;
    let as_json = as_json.as_str();
    fs::write("./ckb-hashes.json", as_json)?;
    Ok(decoded)
}
