use includedir_codegen::Compression;

fn main() {
    includedir_codegen::start("DAPP_FILES")
        .dir("templates/trampoline", Compression::Gzip)
        .build("templates.rs")
        .unwrap();
    includedir_codegen::start("CRA_FILES")
        .dir("templates/dapp", Compression::Gzip)
        .build("cra_template.rs")
        .unwrap();
}
