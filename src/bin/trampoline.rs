use anyhow::Result;
use ckb_types::H256;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str::FromStr;
use structopt::StructOpt;
use trampoline::handlers::{create_ckb_dapp, deploy, faucet, new_project, pw_config};
use trampoline::opts::{DeployCommands, Opts, TrampolineCommand};
use trampoline::rpc::{
    display_cached_tx_info, get_cached_tx_info, get_pw_tx_info, get_sudt_tx_info, get_tx_info,
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
                let res = get_pw_tx_info(DEV_RPC_URL)?;
                println!("{:?}", res);
            }
            "sudt" => {
                let res = get_sudt_tx_info(DEV_RPC_URL);
                println!("{:?}", res);
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
            let chain_config = pw_config::ChainConfig::new(&config)?;
            let dapp_config = pw_config::gen_config(&chain_config)?;
            config.save_dapp_config(dapp_config)?;
        }
        TrampolineCommand::DappConfig { name, save } => {
            let dapp_script_ref = pw_config::gen_custom_cell_config(&name, &config)?;
            println!(
                "Script Ref Generated: {}\nAt Path: {}",
                serde_json::to_string(&dapp_script_ref.0)?,
                &dapp_script_ref.1.to_str().unwrap()
            );
        }
        TrampolineCommand::CreateCkbDapp { name } => {
            create_ckb_dapp::create(name.clone());
            config.trampoline.dapp_name = Some(name);
            config.save()?;
        }
        TrampolineCommand::Faucet { target, amount } => {
            let container_name = &config.trampoline.name;
            faucet::transfer_from_genesis(
                target.as_str(),
                container_name.as_str(),
                amount.as_str(),
            )?;
        }
        TrampolineCommand::Deploy { deploy_plan } => match deploy_plan {
            DeployCommands::Contract { contract } => {
                let container_name = &config.trampoline.name;
                let contract_path = contract.contract_path.as_str();
                let contract_name = contract.contract_name.as_str();

                let hash =
                    deploy::deploy_local(contract_name, container_name.as_str(), contract_path)?;
                println!("Hash of Deployment Transaction: {}", hash);

                let mut new_contract = trampoline::TrampolineContract {
                    name: contract_name.to_string(),
                    path: contract_path.to_string(),
                    tx_hash: Some(H256::from_str(
                        hash.as_str().trim_end().trim_start_matches("0x"),
                    )?),
                    data_hash: None,
                    type_hash: None,
                };

                new_contract.data_hash = Some(new_contract.to_data_hash()?);

                config.add_contract(new_contract)?.save()?;
            }
            _ => {}
        },
        TrampolineCommand::HealthCheck => {
            let context = trampoline::load_context()?;
            println!("Loaded context: {:?}", context);
        }
        TrampolineCommand::GetTx { hash } => {
            let mut hash = hash.as_str();
            if hash.starts_with("0x") {
                hash = hash.trim_start_matches("0x");
            }
            let tx_hash = H256::from_str(hash)?;

            let tx = get_tx_info("http://localhost:8114/", tx_hash)?;
            let to_disp = serde_json::json!(tx);
            println!("{}", to_disp);
        }

        TrampolineCommand::Start {
            with_git,
            with_deploy_scripts,
        } => {
            if with_git {
                let _git_init = Command::new("make")
                    .arg("initialize")
                    .stderr(Stdio::inherit())
                    .spawn()?
                    .wait()?;
            }

            if with_deploy_scripts {
                let _build_proc = Command::new("make")
                    .arg("scripts")
                    .stderr(Stdio::inherit())
                    .spawn()?
                    .wait()?;
            }
            let _docker = Command::new("make")
                .arg("start-docker")
                .stderr(Stdio::inherit())
                .spawn()?
                .wait()?;
            let sleep_time = std::time::Duration::from_secs(2);
            std::thread::sleep(sleep_time);
            let _ckb = Command::new("make")
                .arg("start-ckb")
                .stderr(Stdio::inherit())
                .spawn()?
                .wait()?;
            std::thread::sleep(sleep_time);
            let _miner = Command::new("make")
                .arg("start-miner")
                .stderr(Stdio::inherit())
                .spawn()?
                .wait()?;
            std::thread::sleep(sleep_time);
            let _indexer = Command::new("make")
                .arg("start-indexer")
                .stderr(Stdio::inherit())
                .spawn()?
                .wait()?;
        }
        TrampolineCommand::DeployDefaultScripts {} => {
            let scripts_deployed = Command::new("docker")
                .args([
                    "exec",
                    "-w",
                    "/trampoline/deployed",
                    format!("{}", &config.trampoline.name).as_str(),
                    "ls",
                ])
                .stderr(Stdio::inherit())
                .output()?;

            let _output = String::from_utf8(scripts_deployed.stdout)?;
            Command::new("make")
                .arg("deploy-pw-lock-local")
                .stderr(Stdio::inherit())
                .spawn()?
                .wait()?;
            let sleep_time = std::time::Duration::from_secs(5);
            std::thread::sleep(sleep_time);

            Command::new("make")
                .arg("deploy-sudt-local")
                .stderr(Stdio::inherit())
                .spawn()?
                .wait()?;

            std::thread::sleep(sleep_time);
            let chain_config = pw_config::ChainConfig::new(&config)?;
            let dapp_config = pw_config::gen_config(&chain_config)?;
            config.save_dapp_config(dapp_config)?;
        }
        _ => {
            println!("No other commands yet");
        }
    }
    Ok(())
}
