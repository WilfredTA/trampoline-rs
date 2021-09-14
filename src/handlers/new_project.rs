use crate::handlers::TEMPLATES;
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use tera::{self, Context as TeraContext};

#[derive(Debug, Clone)]
pub struct Project {
    path: PathBuf,
    name: String,
}

pub fn generate_project<P: AsRef<Path>>(name: String, project_path: P) -> Result<()> {
    let mut proj_path = std::env::current_dir()?;
    let mut real_path = PathBuf::new();
    real_path.push(project_path);
    real_path.push(&name);

    fs::create_dir(&real_path)?;

    let mut sub_dir_path = PathBuf::new();
    sub_dir_path.push(&real_path);
    sub_dir_path.push("dapp");
    fs::create_dir(&sub_dir_path)?;
    sub_dir_path.pop();
    sub_dir_path.push(".trampoline");
    fs::create_dir(&sub_dir_path)?;
    for trampoline_subdir in &["deployed", "transactions"] {
        let mut t_path = sub_dir_path.clone();
        t_path.push(trampoline_subdir);
        fs::create_dir(&t_path)?;
    }
    sub_dir_path.pop();
    sub_dir_path.push("deps");
    fs::create_dir(&sub_dir_path)?;
    sub_dir_path.pop();
    sub_dir_path.push("ckb_dev");
    fs::create_dir(&sub_dir_path)?;
    for ckb_dev_dirs in ["data", "specs", "dev_keys"] {
        let mut dev_path = sub_dir_path.clone();
        dev_path.push(&ckb_dev_dirs);
        fs::create_dir(dev_path)?;
    }
    sub_dir_path.pop();
    let mut context = TeraContext::new();
    context.insert("PROJ_NAME", &name);
    proj_path.push(&name);
    context.insert("BASE_PROJ_PATH", &proj_path.as_path().to_str());
    for path in TEMPLATES.get_template_names() {
        if path.starts_with("dapp") {
            continue;
        }
        while !&sub_dir_path.ends_with(&name) {
            sub_dir_path.pop();
        }
        match path {
            "Makefile.template" => {
                sub_dir_path.push("Makefile");
            }
            "Dockerfile.template" => {
                sub_dir_path.push("Dockerfile");
            }
            _ => {
                sub_dir_path.push(&path);
            }
        }
        let content = TEMPLATES.render(path, &context)?;
        let err_message = format!(
            "Error writing to {:?} with template {} in project {}",
            &sub_dir_path.to_str().unwrap(),
            path,
            &name
        );
        fs::write(&sub_dir_path, content).expect(err_message.as_str());
        sub_dir_path.pop();
    }
    Ok(())
}
