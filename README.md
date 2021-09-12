# Trampoline-rs

The framework for building powerful dApps on the number one UTXO chain, Nervos Network CKB.

*This is an early-stage, currently very incomplete port of IAMM Network's Trampoline to Rust*

*Why port it to Rust? Because I was tired of NPM*

## Installation
`cargo install --git https://github.com/wilfredTA/trampoline trampoline`

## Usage
All you can do right now is create a new project:
```bash
trampoline new <project_name>
```

This will create a new directory with `<project_name>`.

It generates a Makefile and Dockerfile for development purposes.
Currently, each Make recipe is small and single purpose, so it takes a few different commands to get started.

```bash
cd <project_name>

make initialize
```

Then run `make pw-lock`. This will fail with an error the first time... so just execute it again and it will succeed.

Then, here are the following recipes available:

1. `make ckb-scripts`: This builds important scripts such as the SUDT script and the open tx lock.
2. `make start-docker`: This sets up the docker environment in which the local developer network will run
3. `make start-ckb`: This starts a local ckb node configured for Trampoline development
4. `make start-miner`: Starts mining the local ckb node within docker
5. `make deploy-<script_name>-local`: Deploys a prebuilt script to the dev network and caches information about it so
that trampoline can generate the correct bindings for the javascript dapp. Currently you can do 
`make deploy-pw-lock-local` and `make deploy-sudt-local`.