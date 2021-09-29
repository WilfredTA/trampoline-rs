use structopt::StructOpt;

use anyhow::Result;
use ckb_types::H256;
use std::path::PathBuf;
use std::str::FromStr;
use trampoline::handlers::{create_ckb_dapp, deploy, new_project, pw_config, faucet};
use trampoline::opts::{
    ContractDeployInfo, DappDeployInfo, DeployCommands, Opts, TrampolineCommand,
};
use trampoline::rpc::{
    display_cached_tx_info, get_cached_tx_info, get_pw_tx_info, get_sudt_tx_info,
};
use trampoline::DEV_RPC_URL;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::from_args();

    let mut config = trampoline::load_context()?;
    match opts.sub {
        TrampolineCommand::NewProject { name } => {
            let proj_path = PathBuf::new();
            new_project::generate_project(name, &proj_path)?;
        }
        TrampolineCommand::GetDeployed { name } => match name.as_str() {
            "pwlock" => {
                get_pw_tx_info(DEV_RPC_URL);
            }
            "sudt" => {
                get_sudt_tx_info(DEV_RPC_URL);
            }
            _ => {
                let base_path = config.trampoline.path;

                display_cached_tx_info(
                    DEV_RPC_URL,
                    format!("{}/.trampoline/deployed/{}-tx", base_path, name),
                );
            }
        },
        TrampolineCommand::PwConfig => {
            pw_config::read_hash_toml()?;
            pw_config::gen_config()?;
        }
        TrampolineCommand::CreateCkbDapp { name } => {
            create_ckb_dapp::create(name);
        }
        TrampolineCommand::Faucet {target, amount} => {
            let container_name = &config.trampoline.name;
            faucet::transfer_from_genesis(target.as_str(), container_name.as_str(), amount.as_str())?;
        }
        TrampolineCommand::Deploy { deploy_plan } => match deploy_plan {
            DeployCommands::Contract { contract } => {
                let container_name = &config.trampoline.name;
                let contract_path = contract.contract_path.as_str();
                let contract_name = contract.contract_name.as_str();

                let hash =
                    deploy::deploy_local(contract_name, container_name.as_str(), contract_path)?;
                println!("Hash of Deployment Transaction: {}", hash);

                let new_contract = trampoline::TrampolineContract {
                    name: contract_name.to_string(),
                    path: contract_path.to_string(),
                    tx_hash: Some(H256::from_str(
                        hash.as_str().trim_end().trim_start_matches("0x"),
                    )?),
                    data_hash: None,
                    type_hash: None,
                };

                config.add_contract(new_contract)?.save()?;
            }
            _ => {}
        },
        TrampolineCommand::HealthCheck => {
            let context = trampoline::load_context()?;
            println!("Loaded context: {:?}", context);
        }
        _ => {
            println!("No other commands yet");
        }
    }
    Ok(())
}
