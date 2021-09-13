use std::process::{self, Command, Stdio};
use std::fs;
use anyhow::Result;
pub fn create(name: String, path_to_template: &str) -> Result<()> {
    let mut cra_res = Command::new("npx")
        .args(["create-react-app", format!("{}", name).as_str(), "--template", format!("file:{}", path_to_template).as_str()])
        .stdout(Stdio::inherit())
        .spawn()?
        .wait()?;


    Ok(())
}