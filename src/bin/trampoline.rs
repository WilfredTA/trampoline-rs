use structopt::StructOpt;

use anyhow::Result;
use std::path::PathBuf;
use trampoline::handlers::new_project;
use trampoline::opts::{Opts, TrampolineCommand};
#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::from_args();

    match opts.sub {
        TrampolineCommand::NewProject { name } => {
            let proj_path = PathBuf::new();
            new_project::generate_project(name, &proj_path)?;
        }
        _ => {
            println!("No other commands yet");
        }
    }
    Ok(())
}
