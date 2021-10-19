# Trampoline-rs

The framework for building powerful dApps on the number one UTXO chain, Nervos Network CKB.

*This software is currently in early-stage, pre-alpha development*

## Installation

### Pre-requisites
- Rust toolchain
- Docker

Note: You must have permission to manage docker. [This article can help](https://docs.docker.com/engine/install/linux-postinstall/) if you run into permissions errors
when running docker.

`cargo install trampoline --git https://github.com/WilfredTA/trampoline-rs`

Or, clone the project, then `cd trampoline-rs && cargo build && cargo install --path . `
## Usage
All you can do right now is create a new project, launch a containerized developer netowrk, autodeploy useful scripts
that aren't included in the genesis cells, and generate configs that make building dapps a lot easier.

### Create a new project
To get started:
```bash
trampoline new <project_name>
```

This will create a new directory with `<project_name>`.

### Launch developer environment
Navigate to your project's directory and you can get started with two commands:

```bash
trampoline start --with-git-init --with-deploy-scripts
```

### Deploy useful scripts

Useful scripts that may be used by dapp developers include:
1. pw-lock: This script enables dapp users to sign transactions with their ethereum keys, thereby enabling tx signing with, e.g., metamask
2. sudt: This script enables the creation of SUDTs - or simple user defined tokens - the ckb analogue of ERC20 fungible tokens.

To deploy these to chain, ensure you've already run `trampoline start`. Then, run `trampoline deploy-default-scripts`.

You can check the status of the deployment transactions with `trampoline deployed sudt` and `trampoline deployed pwlock`.


### Generate dapp config
Your dapp config is a variation of the config file expected by PW-SDK. PW-SDK is a useful tool for building front-end 
apps that interact with ckb, and enables metamask compatibility.

After scripts are deployed, you can run `trampoline pwconfig` to generate a `PwConfig.json` file. 

which can be moved to your trampoline project's `dapp/<dapp_name>/src` directory for use in your front end.

If you've created your dapp already with `trampoline create-ckb-dapp <dapp_name>`, then this file will be automatically
placed in your dapp's src folder for you.


### Deploy custom scripts

`trampoline deploy --name my_awesome_smart_contract --path /path/to/compiled/contract`

This will also generate the configuration files necessary to interact with your script on the dapp side. This includes:
1. The outpoint of the script
2. The `Script` struct to attach to cells that use the deployed contract
3. The data hash and type hash of the script



### React-based Dapp

After running the `make` recipes in the root of your Trampoline project, you can generate a React dApp frontend like so:

`trampoline create-ckb-dapp <front_end_name>`
