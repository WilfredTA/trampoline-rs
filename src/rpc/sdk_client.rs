use anyhow::{Context, Result};
use ckb_sdk::rpc::{CellDep, OutPoint, Script, TransactionView, TransactionWithStatus};
use ckb_sdk::HttpRpcClient;
use ckb_types::H256;
use std::fs;
use std::path::Path;
use std::str::FromStr;

pub fn get_pw_tx_info(url: &str) -> Result<TransactionWithStatus> {
    get_cached_tx_info(url, "./.trampoline/deployed/pwlock-tx")
}

pub fn get_sudt_tx_info(url: &str) -> Result<TransactionWithStatus> {
    get_cached_tx_info(url, "./.trampoline/deployed/sudt-tx")
}

fn get_cached_tx_info<P: AsRef<Path>>(url: &str, path: P) -> Result<TransactionWithStatus> {
    let mut client = HttpRpcClient::new(url.to_string());
    let tx_hash = read_cached_tx_hash(path)?;
    let tx_view = client.get_transaction(tx_hash).unwrap();
    println!("Transaction retrieved: {:?}", tx_view);
    Ok(tx_view.unwrap())
}

pub fn read_cached_tx_hash<P: AsRef<Path>>(path: P) -> Result<H256> {
    let tx_hash = fs::read_to_string(path)?;
    let tx_hash = tx_hash.trim_end().trim_start_matches("0x");
    Ok(H256::from_str(tx_hash)?)
}
