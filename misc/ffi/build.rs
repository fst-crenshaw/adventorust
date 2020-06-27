use std::env;
use std::path::PathBuf;

fn main() {
    let project_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
   
    let snappy_dir = project_root.join("vendor").join("snappy");

    // The following is equilvalent to the "-L" flag in rustc.  It
    // indicates the directory in which cargo ought to search for
    // libraries to link with ~/src/.  In this case, Link ~/src/ with
    // libraries located in ~/vendor/snappy/build.
    println!("cargo:rustc-link-search={}", snappy_dir.join("build").display());    
    // The following is equivalent to the "-l" flag in rustc.  It
    // indicates the library with which cargo ought to link
    // with ~/src/.
    println!("cargo:rustc-link-lib=snappy");

}

