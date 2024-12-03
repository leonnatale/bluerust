use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=BluetoothApis");
    let bindings = bindgen::Builder::default()
        .header("src/bindings/wrapper.h")
        .allowlist_function("Bluetooth.*")
        .allowlist_type("BLUETOOTH_.*")
        .allowlist_var("BLUETOOTH_.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bluetooth_bindings.rs"))
        .expect("Couldn't write bindings!");
}
