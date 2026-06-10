use std::env;
use std::path::PathBuf;

fn main() {
    let dir = env::current_dir().unwrap();
    let asn1_lib_path = dir.parent().unwrap().join("asn1_lib");
    
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=native={}", asn1_lib_path.display());
    
    // Tell cargo to tell rustc to link the system shared library `asn1` (which is `libasn1.so`)
    println!("cargo:rustc-link-lib=dylib=asn1");
    
    // Also add the path to the LD_LIBRARY_PATH when running (helpful for `cargo run`)
    // Actually cargo doesn't set LD_LIBRARY_PATH from here, we will handle it in the wrapper script.
}
