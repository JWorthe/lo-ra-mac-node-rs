extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // TODO: We need to compile the C LoRaMac-node project here. Any
    // artifacts should be put under `out_path`.
    // See https://docs.rs/cmake/0.1.45/cmake/

    // TODO: Once we've built the C library, we need to tell rustc to link the C library
    //println!("cargo:rustc-link-lib=static=???");

    let bindings = bindgen::Builder::default()
        .header("LoRaMac-node/src/mac/LoRaMac.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_arg("-ILoRaMac-node/src/mac")
        .clang_arg("-ILoRaMac-node/src/boards")
        .clang_arg("-ILoRaMac-node/src/system")
        .clang_arg("-ILoRaMac-node/src/radio")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
