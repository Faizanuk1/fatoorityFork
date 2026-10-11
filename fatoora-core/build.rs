//! Losslessly compress bundled schemas for WASM; native resource bytes stay direct.
use std::{env, fs, path::Path};

fn compress_directory(source: &Path, output: &Path) {
    fs::create_dir_all(output).expect("create schema output directory");
    let mut entries: Vec<_> = fs::read_dir(source)
        .expect("read bundled schema directory")
        .map(|entry| entry.expect("read schema entry").path())
        .collect();
    entries.sort();
    for path in entries {
        let destination = output.join(path.file_name().expect("schema name"));
        if path.is_dir() {
            compress_directory(&path, &destination);
        } else if path.extension().is_some_and(|extension| extension == "xsd") {
            println!("cargo:rerun-if-changed={}", path.display());
            let original = fs::read(&path).expect("read schema");
            let compressed = miniz_oxide::deflate::compress_to_vec_zlib(&original, 9);
            let restored = miniz_oxide::inflate::decompress_to_vec_zlib(&compressed)
                .expect("decompress generated schema");
            assert_eq!(original, restored, "schema compression must be byte-exact");
            fs::write(destination, compressed).expect("write compressed schema");
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if env::var("CARGO_CFG_TARGET_ARCH").as_deref() != Ok("wasm32") {
        return;
    }
    let schemas = Path::new("assets/schemas/UBL2.1/xsd");
    println!("cargo:rerun-if-changed={}", schemas.display());
    let output = env::var_os("OUT_DIR").expect("Cargo OUT_DIR");
    compress_directory(schemas, &Path::new(&output).join("schemas"));
}
