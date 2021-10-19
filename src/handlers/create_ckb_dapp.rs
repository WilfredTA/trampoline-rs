use crate::handlers::TEMPLATES;
use anyhow::Result;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str::FromStr;
use tempfile::tempdir;
use tera::Context;

pub fn create(name: String) -> Result<()> {
    let mut dapp_path = std::env::current_dir()?;
    dapp_path.push(name.as_str());
    let cra_template_dir = tempdir()?;

    let mut cra_template_path = PathBuf::from(cra_template_dir.path());
    let mut path_to_template = cra_template_path.clone();
    let mut context = Context::new();
    cra_template_path.push("dapp_template");
    context.insert("DAPP_NAME", &name);
    context.insert("dapp_name", &name);
    for file in TEMPLATES.get_template_names() {
        if file.starts_with("dapp/dapp_template") {
            while !&cra_template_path.ends_with("dapp_template") {
                cra_template_path.pop();
            }
            let shortened_file_name = file.strip_prefix("dapp/dapp_template/").unwrap();

            let contents = TEMPLATES.render(file, &context)?;
            create_nonexistent_dirs(&Path::new(shortened_file_name), &cra_template_path)?;
            cra_template_path = cra_template_path.join(shortened_file_name);

            let err_message = format!(
                "Error writing to {:?} with template {} in project",
                &cra_template_path.to_str().unwrap(),
                file,
            );
            let mut file = fs::File::create(&cra_template_path.as_path())?;
            file.write_all(contents.as_bytes())
                .expect(err_message.as_str());
            fs::write(&cra_template_path, contents).expect(err_message.as_str());
        }
    }
    path_to_template.push("dapp_template");

    write_dapp(&path_to_template, name.clone())?;
    let config = fs::read_to_string("./PwConfig.json")?;
    // config = config.to_case(Case::Camel);
    fs::write(format!("./{}/src/pwConfig.json", name), config)?;

    Ok(())
}

fn is_file<P: AsRef<Path>>(check: P) -> bool {
    let check = check.as_ref().to_str().unwrap();
    check.ends_with(".ts")
        || check.ends_with(".js")
        || check.ends_with(".css")
        || check.ends_with(".tsx")
        || check.ends_with(".jsx")
        || check.ends_with(".svg")
        || check.ends_with(".md")
        || check.ends_with(".html")
        || check.ends_with(".txt")
        || check.ends_with(".json")
        || check.ends_with("gitignore")
}
fn create_nonexistent_dirs(path_to: &Path, base_dir: &Path) -> Result<()> {
    let mut curr_path = PathBuf::new();
    //curr_path.push(base_dir);
    let full_path = base_dir.join(path_to.to_path_buf());
    full_path.iter().for_each(|section| {
        curr_path.push(section);

        if is_file(&curr_path) {
            return;
        }
        if !curr_path.exists() {
            let err_string = format!("Unable to create subpath {}", curr_path.to_str().unwrap());
            fs::create_dir(&curr_path).expect(err_string.as_str());
        }
    });

    Ok(())
}
fn write_dapp(path_to_template: &Path, name: String) -> Result<()> {
    println!("PATH TO TEMPLATE: {:?}", path_to_template);
    let curr_dir = std::env::current_dir()?;
    if !curr_dir.ends_with("dapp") {
        std::env::set_current_dir(curr_dir.join("dapp"));
    }
    Command::new("npx")
        .args([
            "create-react-app",
            format!("{}", name).as_str(),
            "--template",
            format!("file:{}", path_to_template.to_str().unwrap()).as_str(),
        ])
        .stdout(Stdio::inherit())
        .spawn()?
        .wait()?;

    Ok(())
}
