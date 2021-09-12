use lazy_static::lazy_static;
use tera::{self, Tera};

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();
        for path in DAPP_FILES.file_names() {
            let name = path
                .strip_prefix("templates/")
                .expect("Failed to remove prefix");
            let content = {
                let file_contents = DAPP_FILES.get(path).expect("read template");
                String::from_utf8(file_contents.to_vec()).expect("template contents")
            };

            tera.add_raw_template(name, &content)
                .expect("failed to add template");
        }
        tera
    };
}

pub mod new_project;
pub mod pw_config;
