use lazy_static::lazy_static;
use tera::{self, Tera};

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
include!(concat!(env!("OUT_DIR"), "/cra_template.rs"));

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();
        for path in DAPP_FILES.file_names() {
            let name = path
                .strip_prefix("templates/trampoline/")
                .expect("Failed to remove prefix");
            let content = {
                let file_contents = DAPP_FILES.get(path).expect("read template");
                String::from_utf8(file_contents.to_vec()).expect("template contents")
            };

            tera.add_raw_template(name, &content)
                .expect("failed to add template");
        }

        for path in CRA_FILES.file_names() {
            if path.ends_with(".ico") || path.ends_with(".png") {
                continue;
            }
            let name = path
                .strip_prefix("templates/")
                .expect("Failed to remove prefix in cra template");
            let content = {
                let file_contents = CRA_FILES.get(path).expect("read template");
                String::from_utf8(file_contents.to_vec()).unwrap_or_else(|err| {
                    println!("Got error when decoding utf8 content in file {}", name);
                    return String::default();
                })
            };
            let err_message = format!("Error adding template file: {} with name {}", path, name);
            tera.add_raw_template(name, &content)
                .expect(err_message.as_str());
        }
        tera
    };
}

pub mod create_ckb_dapp;
pub mod deploy;
pub mod new_project;
pub mod pw_config;
pub mod pw_transaction;
pub mod faucet;
