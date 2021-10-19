use crate::rpc::{get_pw_tx_info, get_sudt_tx_info};
use crate::{TrampolineConfig, DEV_RPC_URL};
use anyhow::Result;
use ckb_jsonrpc_types::{DepType, JsonBytes, ScriptHashType, Uint32};

use ckb_types::{bytes::Bytes, prelude::*, H256};
use serde::{Deserialize, Serialize};
use serde_json;

use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use toml;

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Script {
    code_hash: H256,
    hash_type: ScriptHashType,
    args: JsonBytes,
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OutPoint {
    tx_hash: H256,
    index: Uint32,
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CellDep {
    out_point: OutPoint,
    dep_type: DepType,
}
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwScriptRef {
    cell_dep: CellDep,
    script: Script,
}
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DappConfig {
    dev: PwConfig,
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

impl ChainConfig {
    pub fn new(project: &TrampolineConfig) -> Result<Self> {
        let root_path = PathBuf::from_str(&project.trampoline.path)?;
        let toml_ = fs::read_to_string(root_path.join("ckb-hashes.toml"))?;
        let decoded: ChainConfig = toml::from_str(toml_.as_str())?;
        Ok(decoded)
    }

    pub fn save_as_json(&self, project: &TrampolineConfig) -> Result<()> {
        let root_path = PathBuf::from_str(&project.trampoline.path)?;
        let as_json = serde_json::to_string(&self)?;
        let as_json = as_json.as_str();
        fs::write(root_path.join("ckb-hashes.json"), as_json)?;
        Ok(())
    }
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

pub fn gen_custom_cell_config(
    name: &str,
    proj_config: &TrampolineConfig,
) -> Result<(PwScriptRef, PathBuf)> {
    let mut path_to = PathBuf::from_str(&proj_config.trampoline.path)?;
    path_to = path_to
        .join("contract_configs")
        .join(format!("{}.json", name));
    println!("PATH TO SAVE FILE: {:?}", path_to);
    if let Some(contracts) = &proj_config.contracts {
        let target = contracts.iter().find(|contract| contract.name == name);
        if let Some(contract) = target {
            let contract_outpoint = OutPoint {
                tx_hash: contract.tx_hash.as_ref().unwrap().clone(),
                index: Uint32::from(0),
            };
            let contract_cell_dep = CellDep {
                out_point: contract_outpoint,
                dep_type: DepType::Code,
            };

            let contract_script = Script {
                code_hash: contract.data_hash.as_ref().unwrap().clone(),
                hash_type: ScriptHashType::Data,
                args: JsonBytes::default(),
            };
            let finalized = PwScriptRef {
                cell_dep: contract_cell_dep,
                script: contract_script,
            };

            let script_ref_contents = serde_json::to_string(&finalized)?;
            fs::write(&path_to, &script_ref_contents)?;
            path_to.pop();
            path_to.pop();

            if let Some(dapp_name) = &proj_config.trampoline.dapp_name {
                path_to = path_to
                    .join(format!("dapp/{}/src", dapp_name))
                    .join(format!("{}.json", name));
                fs::write(&path_to, &script_ref_contents)?;
            }
            Ok((finalized, path_to))
        } else {
            Err(anyhow::Error::msg(
                "Tried to generate a config for a script that is not yet deployed",
            ))
        }
    } else {
        Err(anyhow::Error::msg(
            "No deployed contracts found in trampoline.toml file",
        ))
    }
}
// To do: Remove using ckb_sdk types and take advantage of serialization implementations.
// ckb_sdk types do not serialize exactly as required
pub fn gen_config(chain_config: &ChainConfig) -> Result<DappConfig> {
    let sys_cells = &chain_config.ckb_dev.system_cells;
    let dao_type = gen_syscell_config(chain_config, SysCellSelected::DaoType)?;
    let default_lock = gen_syscell_config(chain_config, SysCellSelected::DefaultLock)?;
    let sudt_type = gen_syscell_config(chain_config, SysCellSelected::Sudt)?;
    let pw_lock = gen_syscell_config(chain_config, SysCellSelected::PwLock)?;
    let multi_sig_lock = gen_multisig_config()?;
    let acp_lock_list = gen_acp_lock_list_config()?;

    let pw_config = PwConfig {
        dao_type,
        default_lock,
        pw_lock,
        sudt_type,
        multi_sig_lock,
        acp_lock_list,
    };

    let dapp_config = DappConfig { dev: pw_config };

    // fs::write("./PwConfig.json", serde_json::to_string(&dapp_config)?)?;
    Ok(dapp_config)
}

fn gen_acp_lock_list_config() -> Result<Vec<Script>> {
    let mut vec = Vec::new();
    let script = Script {
        code_hash: Default::default(),
        args: Default::default(),
        hash_type: ScriptHashType::Type,
    };
    vec.push(script);

    Ok(vec)
}
fn gen_multisig_config() -> Result<PwScriptRef> {
    let out_point = OutPoint {
        tx_hash: H256::from_str(
            "d6d78382f948a6fab16ba084a4c3ed16eb3fe203669a6bc8a8f831e09177117f",
        )?,
        index: Uint32::from(1),
    };

    let cell_dep = CellDep {
        out_point,
        dep_type: DepType::DepGroup,
    };

    let script = Script {
        code_hash: H256::from_str(
            "5c5069eb0857efc65e1bca0c07df34c31663b3622fd3876c876320fc9634e2a8",
        )?,
        hash_type: ScriptHashType::Type,
        args: Default::default(),
    };

    Ok(PwScriptRef { cell_dep, script })
}
fn gen_syscell_config(sys_cells: &ChainConfig, type_: SysCellSelected) -> Result<PwScriptRef> {
    match type_ {
        SysCellSelected::DaoType => gen_dao_config(sys_cells),
        SysCellSelected::DefaultLock => gen_default_lock_config(sys_cells),
        SysCellSelected::Sudt => gen_sudt_config(),
        SysCellSelected::PwLock => gen_pwlock_config(),
    }
}

fn gen_dao_config(sys_cells: &ChainConfig) -> Result<PwScriptRef> {
    let sys_cells = &sys_cells.ckb_dev.system_cells;
    let raw_dao_out = RawOutpoint {
        tx_hash: sys_cells[1].tx_hash.clone(),
        code_hash: sys_cells[1].type_hash.as_ref().unwrap().clone(),
        index: sys_cells[1].index,
    };

    let dao_script = build_script(&raw_dao_out.code_hash, "type", "0x")?;
    let dao_cell_dep = build_cell_dep(&raw_dao_out.tx_hash, raw_dao_out.index, "code")?;

    let dao_pw_obj = PwScriptRef {
        script: dao_script,
        cell_dep: dao_cell_dep,
    };
    // fs::write("./pw-config-dao.json", serde_json::to_string(&dao_pw_obj)?)?;

    Ok(dao_pw_obj)
}

fn gen_default_lock_config(sys_cells: &ChainConfig) -> Result<PwScriptRef> {
    let dep_groups = &sys_cells.ckb_dev.dep_groups;
    let sys_cells = &sys_cells.ckb_dev.system_cells;
    let raw_lock_out = RawOutpoint {
        tx_hash: dep_groups[0].tx_hash.clone(),
        code_hash: sys_cells[0].type_hash.as_ref().unwrap().clone(),
        index: dep_groups[0].index,
    };

    let lock_script = build_script(&raw_lock_out.code_hash, "type", "0x")?;
    let lock_dep = build_cell_dep(&raw_lock_out.tx_hash, raw_lock_out.index, "group")?;

    let lock_pw_obj = PwScriptRef {
        script: lock_script,
        cell_dep: lock_dep,
    };

    // fs::write(
    //     "./pw-config-default-lock.json",
    //     serde_json::to_string(&lock_pw_obj)?,
    // )?;

    Ok(lock_pw_obj)
}

// To do: code_hash should be hash of the script attached to sudt type output, not
// the code hash contained within the sudt output type script.
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
    let args = sudt_info.transaction.inner.outputs[0]
        .type_
        .as_ref()
        .unwrap()
        .args
        .clone();

    // let args = hex::decode(args.trim_start_matches("0x"))?;
    // let args = Bytes::copy_from_slice(args.as_slice());
    // println!("Args as bytes: {:?}", args);
    // let args = JsonBytes::from_bytes(args);

    let script = Script {
        code_hash,
        hash_type: ScriptHashType::Type,
        args,
    };
    let script = gen_script_dep(script)?;

    let index = Uint32::from(index);
    let out_point = OutPoint { tx_hash, index };

    let cell_dep = CellDep {
        out_point,
        dep_type: DepType::Code,
    };

    let sudt_pw_obj = PwScriptRef { script, cell_dep };
    //
    // fs::write(
    //     "./pw-config-sudt.json",
    //     serde_json::to_string(&sudt_pw_obj)?,
    // )?;
    Ok(sudt_pw_obj)
}

pub fn gen_script_dep(type_script_on_dep: Script) -> Result<Script> {
    // args as default
    // code_hash as the hash of the input script
    // hashtype = type
    // To hash the script, first must serialize the script appropriately.
    // First, Byte32 as code hash
    // Second, Byte as hash_type,
    // Third, Bytes as args
    let mut script = ckb_jsonrpc_types::Script::default();
    script.hash_type = type_script_on_dep.hash_type;
    script.code_hash = type_script_on_dep.code_hash;
    script.args = type_script_on_dep.args;

    let packed_script = ckb_types::packed::Script::from(script);
    let script_hash = packed_script.calc_script_hash();

    Ok(Script {
        code_hash: script_hash.unpack(),
        hash_type: ScriptHashType::Type,
        args: JsonBytes::default(),
    })
}

fn gen_pwlock_config() -> Result<PwScriptRef> {
    let pwlock_info = get_pw_tx_info(DEV_RPC_URL)?;
    let tx_hash = pwlock_info.clone().transaction.hash;
    let index: u32 = 0;
    // println!("PW LOCK TX: {:?}", pwlock_info.transaction.inner);
    let code_hash = pwlock_info.transaction.inner.outputs[0]
        .type_
        .as_ref()
        .unwrap()
        .code_hash
        .clone();

    println!("CODE HASH: {:?}", code_hash.to_string());

    let args = pwlock_info.transaction.inner.outputs[0]
        .type_
        .as_ref()
        .unwrap()
        .args
        .clone();
    println!("ARGS: {:?}", hex::encode(args.as_bytes()));
    // let args = hex::decode(args.trim_start_matches("0x"))?;
    // let args = Bytes::copy_from_slice(args.as_slice());
    // println!("Args as bytes: {:?}", args);
    // let args = JsonBytes::from_bytes(args);

    // Gen address
    let script = Script {
        code_hash,
        hash_type: ScriptHashType::Type,
        args,
    };

    let script = gen_script_dep(script)?;
    println!("SCRIPT DEP: {:?}", script);
    let index = Uint32::from(index);
    println!("INDEX: {:?}", index);
    let out_point = OutPoint { tx_hash, index };

    let cell_dep = CellDep {
        out_point,
        dep_type: DepType::Code,
    };

    let pw_lock_obj = PwScriptRef { script, cell_dep };

    // fs::write(
    //     "./pw-config-pw-lock.json",
    //     serde_json::to_string(&pw_lock_obj)?,
    // )?;
    Ok(pw_lock_obj)
}
fn build_cell_dep(tx_hash: &str, index: u32, dep_type: &str) -> Result<CellDep> {
    let tx_hash = tx_hash.trim_start_matches("0x");
    let dep_type = match dep_type {
        "code" => DepType::Code,
        "group" => DepType::DepGroup,
        _ => DepType::Code,
    };

    let index = Uint32::from(index);

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
