use structopt::StructOpt;
#[derive(Debug, StructOpt)]
pub enum TrampolineCommand {
    #[structopt(name = "new")]
    #[structopt(about = "Create a new Trampoline project")]
    NewProject { name: String },
    #[structopt(name = "network")]
    #[structopt(about = "Start a local developer network")]
    Network {
        #[structopt(help = "Deploy important scripts to the network after start")]
        default_scripts: String,
    },
    #[structopt(name = "faucet")]
    #[structopt(about = "Send ckbytes from genesis accounts to a personal address")]
    Faucet { target: String, amount: String },
    #[structopt(name = "pwconfig")]
    PwConfig,
    #[structopt(name = "deployed")]
    GetDeployed { name: String },
    #[structopt(name = "create-ckb-dapp")]
    CreateCkbDapp {
        name: String,
        path_to_template: String
    }
}

#[derive(Debug, StructOpt)]
pub struct Opts {
    #[structopt(subcommand)]
    pub sub: TrampolineCommand,
}
