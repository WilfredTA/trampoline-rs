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
    #[structopt(
        name = "pwconfig",
        about = "Generate json config file for built in scripts"
    )]
    PwConfig,
    #[structopt(
        name = "deployed",
        about = "Get transaction information about a deployed contract"
    )]
    GetDeployed { name: String },
    #[structopt(name = "create-ckb-dapp")]
    CreateCkbDapp { name: String },
    #[structopt(
        name = "deploy",
        about = "Deploy a custom contract to local environment"
    )]
    Deploy {
        #[structopt(flatten)]
        deploy_plan: DeployCommands,
    },
    #[structopt(name = "get-tx", about = "Retrieve a transaction by its hash")]
    GetTx { hash: String },
    #[structopt(name = "health-check", about = "Quick check for testing purposes")]
    HealthCheck,
    #[structopt(
        name = "import-dapp",
        about = "Import a front end application into trampoline project"
    )]
    ImportDapp { name: String, path: String },
    #[structopt(
        name = "dapp-config",
        about = "Generate json file for interacting with a deployed contract."
    )]
    DappConfig { name: String, save: Option<bool> },
    #[structopt(name = "start", about = "Start local network")]
    Start {
        #[structopt(long = "with-git-init", parse(from_flag))]
        with_git: bool,
        #[structopt(long = "with-deploy-scripts", parse(from_flag))]
        with_deploy_scripts: bool,
    },
    #[structopt(
        name = "deploy-default-scripts",
        about = "Deploy useful scripts such as sudt & pw-lock"
    )]
    DeployDefaultScripts {},
}
#[derive(Debug, StructOpt)]
pub enum DeployCommands {
    #[structopt(
        name = "contract",
        about = "Deploy your contract to developer environment"
    )]
    Contract {
        #[structopt(flatten)]
        contract: ContractDeployInfo,
    },
    #[structopt(name = "dapp", about = "Deploy your dapp to production")]
    Dapp {
        #[structopt(flatten)]
        dapp: DappDeployInfo,
    },
}
#[derive(Debug, StructOpt)]
pub struct DappDeployInfo {
    #[structopt(long = "host_type", short, about = "How to host your dapp")]
    host: String,
    #[structopt(
        long = "name",
        short,
        requires = "hosted",
        about = "Set the dapp name, which will be deployed to <name>.iamm.network"
    )]
    name: String,
}

#[derive(Debug, StructOpt)]
pub struct ContractDeployInfo {
    #[structopt(long = "path", short = "p")]
    #[structopt(help = "Path to contract to deploy")]
    pub contract_path: String,
    #[structopt(long = "name", short = "n")]
    #[structopt(
        help = "Name of contract (used for caching contract information & transaction hash)"
    )]
    pub contract_name: String,
}
#[derive(Debug, StructOpt)]
pub struct Opts {
    #[structopt(subcommand)]
    pub sub: TrampolineCommand,
}
