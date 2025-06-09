use bootloader::BiosBoot;
use std::{env, path::PathBuf};

fn main() {
    print_kernel_dep();
    build_bios_img();
}

fn print_kernel_dep() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let kernel_dir = manifest_dir.join("../kernel");
    println!("cargo:rerun-if-changed={}", kernel_dir.display());
}

fn build_bios_img() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // set by cargo's artifact dependency feature, see
    // https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#artifact-dependencies
    let kernel = PathBuf::from(std::env::var_os("CARGO_BIN_FILE_KERNEL_kernel").unwrap());

    // write the BIOS img file
    let bios_path = out_dir.join("bios.img");
    BiosBoot::new(&kernel)
        .create_disk_image(&bios_path)
        .unwrap();

    println!("cargo:rustc-env=BIOS_PATH={}", bios_path.display());
}
