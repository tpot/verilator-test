use std::env;
use std::process::Command;
use std::path::PathBuf;

fn main() {
    // Call verilator to transpile to C++
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let out_path = PathBuf::from(&out_dir);

    let verilator_status = Command::new("verilator")
        .arg("-cc")
        .arg("rtl/src/counter.sv")
        .arg("-CFLAGS").arg("-DVL_TIME_CONTEXT")
        .arg("--Mdir")
        .arg(&out_dir)
        .status()
        .expect("Failed to execute Verilator");

    assert!(verilator_status.success(), "Verilator returned non-zero exit status");

    // Build libVcounter and libverilated
    let make_status = Command::new("make")
        .arg("-C")
        .arg(&out_dir)
        .arg("-f")
        .arg("Vcounter.mk")
        .status()
        .expect("Build verilator output failed");

    assert!(make_status.success(), "make returned non-zero exit status");

    // Pull include dir out of pkg-config
    let pkg_output = Command::new("pkg-config")
        .arg("--cflags-only-I")
        .arg("verilator")
        .output()
        .expect("Failed to execute pkg-config");

    let cflags_str = String::from_utf8_lossy(&pkg_output.stdout);

    let verilator_include_dir = cflags_str
        .split_whitespace()
        .find(|flag| flag.starts_with("-I"))
        .map(|flag| flag.trim_start_matches("-I"))
        .expect("Could not find include dir in pkg-config output");

    // Build the C++ bridge
    cxx_build::bridge("src/ffi.rs")
        .include(".")
        .include(out_path)
        .include(verilator_include_dir)
        .file("src/Counter.cpp")
        .compile("cxx-counter");

    println!("cargo::rustc-link-search=native={}", out_dir);
    println!("cargo::rustc-link-lib=static=Vcounter");
    println!("cargo::rustc-link-lib=static=verilated");

    println!("cargo:rerun-if-changed=rtl/src/counter.sv");
    println!("cargo:rerun-if-changed=src/ffi.rs");

    println!("cargo:rerun-if-changed=src/Counter.h");
    println!("cargo:rerun-if-changed=src/Counter.cpp");
}
