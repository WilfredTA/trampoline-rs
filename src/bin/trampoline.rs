use structopt::StructOpt;

use anyhow::Result;
use std::path::PathBuf;
use trampoline::handlers::{new_project, pw_config};
use trampoline::opts::{Opts, TrampolineCommand};
use trampoline::rpc::{get_pw_tx_info, get_sudt_tx_info};
use trampoline::DEV_RPC_URL;
#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::from_args();

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
                panic!("Unsupported argument to trampoline deployed <named_tx>");
            }
        },
        TrampolineCommand::PwConfig => {
            //pw_config::read_hash_toml()?;
            pw_config::gen_config()?;
        }
        _ => {
            println!("No other commands yet");
        }
    }
    Ok(())
}
