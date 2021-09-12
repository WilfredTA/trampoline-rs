
use includedir_codegen::Compression;

fn main() {
    includedir_codegen::start("DAPP_FILES")
        .dir("templates", Compression::Gzip)
        .build("templates.rs")
        .unwrap();
}
