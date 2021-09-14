pub mod handlers;
pub mod opts;
pub mod rpc;
use anyhow::Result;
use ckb_types::H256;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use toml;

pub const DOCKER_IMAGE: &str = "iamm/trampoline-env:latest";
pub const DEV_RPC_URL: &str = "http://127.0.0.1:8114";

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
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrampolineConfig {
    pub trampoline: ProjectDesc,
    pub default_contracts: Vec<SystemContract>,
    pub contracts: Option<Vec<TrampolineContract>>,
}

pub fn load_context() -> Result<TrampolineConfig> {
    let curr_dir = std::env::current_dir()?;
    println!("Curr dir in load context: {:?}", curr_dir);

    let trampoline_config_path = curr_dir.join("trampoline.toml");
    let mut config = TrampolineConfig::default();

    if trampoline_config_path.exists() {
        let raw_conf = fs::read_to_string(trampoline_config_path)?;
        config = toml::from_str::<TrampolineConfig>(raw_conf.as_str())?;
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
}
