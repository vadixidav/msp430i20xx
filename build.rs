use std::{env, fs::File, io::Write, path::PathBuf};

fn main() {
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    for (name, body) in [
        ("device.x", &include_bytes!("device.x")[..]),
        ("memory.x", &include_bytes!("memory.x")[..]),
    ] {
        File::create(out.join(name))
            .unwrap()
            .write_all(body)
            .unwrap();
    }
    println!("cargo:rustc-link-search={}", out.display());
}
