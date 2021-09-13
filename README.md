# Trampoline-rs

The framework for building powerful dApps on the number one UTXO chain, Nervos Network CKB.

*This is an early-stage, currently very incomplete port of IAMM Network's Trampoline to Rust*

*Why port it to Rust? Because I was tired of NPM*

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
make deploy-all
```

The first command will set everything up, including your docker environment & start a local ckb node & miner.
The second command will deploy important scripts and then execute the `trampoline pwconfig` command to generate the
config file for front end dapps that use `pw-core`.

I recommend waiting a few seconds between running `make all` and `make deploy-all`, since the deploy-all recipe
*sometimes* fails if the ckb node is still initializing. Usually, waiting about 2-3 seconds suffices. If 
`make deploy-all` fails, you don't have to do anything besides re-run the command.

### React-based Dapp

After running the `make` recipes in the root of your Trampoline project, you can generate a React dApp frontend like so:

`trampoline create-ckb-dapp <front_end_name>`