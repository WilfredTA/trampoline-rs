use anyhow::Result;
use std::process::{Command, Stdio};

pub fn transfer_from_genesis(target_addr: &str, container_name: &str, amount: &str) -> Result<()> {
    let transfer_string = format!(
        "/ckb/ckb/ckb-cli wallet transfer \
--privkey-path /ckb_dummy_accounts/genesis1 \
--to-address {} \
--tx-fee 0.01 \
--capacity {} \
--skip-check-to-address",
        target_addr, amount
    );
    let docker_to_net = Command::new("docker")
        .args([
            "exec",
            "-w",
            "/ckb/ckb",
            container_name,
            "sh",
            "-c",
            transfer_string.as_str(),
        ])
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;

    Ok(())
}
