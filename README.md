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
make all
make deploy-sudt-local
make deploy-pw-lock-local
```

### Generate dapp config
Your dapp config is a variation of the config file expected by PW-SDK, which is a JavaScript framework for building front ends that interact with CKB.

After scripts are deployed, you can run `trampoline pwconfig` to generate a `PwConfig.json` file, which can be moved to your trampoline project's `dapp/<dapp_name>` directory for use in your front end.


### Deploy custom scripts

`trampoline deploy --name my_awesome_smart_contract --path /path/to/compiled/contract`

This will also generate the configuration files necessary to interact with your script on the dapp side. This includes:
1. The outpoint of the script
2. The `Script` struct to attach to cells that use the deployed contract
3. The data hash and type hash of the script



### React-based Dapp

After running the `make` recipes in the root of your Trampoline project, you can generate a React dApp frontend like so:

`trampoline create-ckb-dapp <front_end_name>`
