pub mod handlers;
pub mod opts;
pub mod rpc;
mod sdk;

use crate::handlers::pw_config::DappConfig;
use anyhow::Result;
use ckb_hash::blake2b_256;
use ckb_types::H256;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub const DOCKER_IMAGE: &str = "iamm/trampoline-env:latest";
pub const DEV_RPC_URL: &str = "http://127.0.0.1:8114";
pub const DEV_MINER_URL: &str = "http://127.0.0.1:8115";
pub const DEV_INDEXER_URL: &str = "http://127.0.0.1:8116";

pub type HashedFieldOpt = Option<H256>;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemContract {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrampolineContract {
    pub name: String,
    pub path: String,
    pub tx_hash: HashedFieldOpt,
    pub data_hash: HashedFieldOpt,
    pub type_hash: HashedFieldOpt,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectDesc {
    pub name: String,
    pub mode: String,
    pub path: String,
    pub dapp_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrampolineConfig {
    pub trampoline: ProjectDesc,
    pub default_contracts: Vec<SystemContract>,
    pub contracts: Option<Vec<TrampolineContract>>,
}

pub fn find_ancestor(curr_path: &mut PathBuf, target: &str) -> Result<Option<PathBuf>> {
    let target_path = curr_path.join(target);
    if target_path.exists() {
        Ok(Some(target_path))
    } else if curr_path.pop() {
        find_ancestor(curr_path, target)
    } else {
        Ok(None)
    }
}

pub fn load_context() -> Result<TrampolineConfig> {
    let curr_dir = std::env::current_dir()?;
    println!("Curr dir in load context: {:?}", curr_dir);

    let mut trampoline_config_path = curr_dir.join("trampoline.toml");
    let mut config = TrampolineConfig::default();

    if trampoline_config_path.exists() {
        let raw_conf = fs::read_to_string(trampoline_config_path)?;
        config = toml::from_str::<TrampolineConfig>(raw_conf.as_str())?;
    } else {
        trampoline_config_path.pop();
        let mut real_path = trampoline_config_path.canonicalize()?;

        let root_trampoline_path = find_ancestor(&mut real_path, "trampoline.toml")?;

        if let Some(proj_path) = root_trampoline_path {
            let raw_conf = fs::read_to_string(proj_path)?;
            config = toml::from_str::<TrampolineConfig>(raw_conf.as_str())?;
        } else {
            config = TrampolineConfig::default();
        }
    }

    Ok(config)
}

impl TrampolineConfig {
    pub fn add_contract(mut self, contract: TrampolineContract) -> Result<Self> {
        self.contracts.get_or_insert(Vec::new()).push(contract);
        Ok(self)
    }

    pub fn save(&self) -> Result<()> {
        let path_to_config = Path::new(self.trampoline.path.as_str()).join("trampoline.toml");
        fs::write(path_to_config, toml::to_string(self)?)?;

        Ok(())
    }

    pub fn save_dapp_config(&self, dapp_config: DappConfig) -> Result<()> {
        let root_project = PathBuf::from_str(&self.trampoline.path)?;
        fs::write(
            root_project.join("PwConfig.json"),
            serde_json::to_string(&dapp_config)?,
        )?;
        if self.trampoline.dapp_name.is_some() {
            fs::write(
                root_project
                    .join("dapp")
                    .join(&self.trampoline.dapp_name.as_ref().unwrap())
                    .join("src")
                    .join("PwConfig.json"),
                serde_json::to_string(&dapp_config)?,
            )?;
        }
        Ok(())
    }
}

impl TrampolineContract {
    pub fn to_data_hash(&self) -> Result<H256> {
        let contract_bytes = fs::read(&self.path)?;
        let hash = blake2b_256(contract_bytes);
        let hash = H256::from(hash);
        Ok(hash)
    }

    pub fn to_data_hash_str(&self) -> Result<String> {
        let raw_hash = self.to_data_hash()?;
        Ok(hex::encode(raw_hash))
    }
}
