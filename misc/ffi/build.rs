use std::env;
use std::path::PathBuf;

fn main() {
    let project_root = {
        let mut buf = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        buf.pop();
        buf
    };

    let snappy_dir = project_root.join("vendor").join("snappy");

    println!(
        "cargo:rustc-link-search={}",
        snappy_dir.join("build").display()
    );    
    println!("cargo:rustc-link-lib=static=snappy");

 }

