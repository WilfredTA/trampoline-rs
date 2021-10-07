use anyhow::Result;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{self, Command, Stdio};
use std::str::FromStr;
use tempfile::{tempdir, tempfile};
use tera::Context;
use serde_json;
use crate::rpc::get_cell_info;

pub fn deploy_local(name: &str, container_name: &str, path: &str) -> Result<String> {
    let local_path = Path::new(path).canonicalize()?;
    let local_path_str = local_path.to_str().unwrap();
    let raw_bytes = fs::read(local_path_str)?;
    let bytes_len = raw_bytes.len();
    println!("Copying raw executable to docker container...");
    copy_to_docker(local_path_str, container_name)?;
    println!("Deploying to local ckb node...");
    deploy_to_net(name, container_name, bytes_len)?;
    println!("Copying deployment transaction record...");
    let tx_hash = copy_tx_to_local(name, container_name)?;

    Ok(tx_hash)
}

fn copy_to_docker(local_path: &str, container_name: &str) -> Result<()> {
    let bin_to_docker = Command::new("docker")
        .args([
            "cp",
            local_path,
            format!("{}:/trampoline/", container_name).as_str(),
        ])
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;

    Ok(())
}

fn deploy_to_net(name: &str, container_name: &str, bytes_len: usize) -> Result<()> {
    println!("DEPLOYING WITH CAPACITY: {}", bytes_len);
    let deploy_command_string = format!(
        "/ckb/ckb/ckb-cli wallet transfer --type-id \
--privkey-path /ckb_dummy_accounts/genesis1 \
--to-address ckt1qyqvsv5240xeh85wvnau2eky8pwrhh4jr8ts8vyj37 \
--tx-fee 0.01 \
--capacity {} \
--to-data-path /trampoline/{} > /trampoline/deployed/{}-tx",
        bytes_len + 300, name, name
    );
    let docker_to_net = Command::new("docker")
        .args([
            "exec",
            "-w",
            "/ckb/ckb",
            container_name,
            "sh",
            "-c",
            deploy_command_string.as_str(),
        ])
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;

    Ok(())
}

fn copy_tx_to_local(name: &str, container_name: &str) -> Result<String> {
    let local_path = format!("./.trampoline/deployed/{}-tx", name);
    let copy_to_local = Command::new("docker")
        .args([
            "cp",
            format!("{}:/trampoline/deployed/{}-tx", container_name, name).as_str(),
            "./.trampoline/deployed/",
        ])
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;

    let tx_hash = fs::read_to_string(local_path.as_str())?;


    Ok(tx_hash)
}
