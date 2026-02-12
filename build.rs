use std::path::Path;
use std::process::Command;

fn run_cmake_command(args: &[&str], current_dir: &String, stage: &str) {
    let status = Command::new("cmake")
        .args(args)
        .current_dir(current_dir)
        .status()
        .unwrap_or_else(|_| panic!("Failed to execute cmake for {stage}"));
    assert!(status.success(), "Draco {stage} failed");
}

fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        println!("cargo:warning=Skipping native build on docs.rs");
        return;
    }

    if std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default() == "wasm32" {
        println!("cargo:warning=Skipping build.rs on wasm32 target");
        return;
    }

    let target = std::env::var("TARGET").unwrap();

    // Step 1: Build Draco with CMake
    let draco_build = "third_party/draco/build".to_string();
    let draco_install = if target.contains("windows-msvc") {
        format!("{draco_build}/Release")
    } else {
        format!("{draco_build}/install")
    };

    if !Path::new(&draco_build).exists() {
        std::fs::create_dir_all(&draco_build).unwrap();
    }

    let status = Command::new("cmake")
        .args([
            "..",
            "-DBUILD_SHARED_LIBS=OFF",
            "-DCMAKE_BUILD_TYPE=Release",
            "-DDRACO_TESTS=OFF",
            &format!("-DCMAKE_INSTALL_PREFIX={}", "install"),
        ])
        .current_dir(&draco_build)
        .status()
        .expect("Failed to run CMake");
    assert!(status.success(), "CMake configuration failed");

    let (build_args, install_args) = if target.contains("windows-msvc") {
        (
            vec!["--build", ".", "--config", "Release"],
            vec!["--install", ".", "--config", "Release"],
        )
    } else {
        (vec!["--build", "."], vec!["--install", "."])
    };

    run_cmake_command(&build_args, &draco_build, "build");
    run_cmake_command(&install_args, &draco_build, "install");

    let mut build = cxx_build::bridge("src/ffi.rs");
    build
        .file("cpp/decoder_api.cc")
        .include("include")
        .include("target/cxxbridge/draco_decoder/src")
        .include("third_party/draco/src")
        .include("third_party/draco/build")
        .include(format!("{draco_install}/include"))
        .flag_if_supported("-std=c++17");

    if target.contains("apple-darwin") {
        build.flag("-mmacosx-version-min=15.5");
    }

    build.compile("decoder_api");

    if target.contains("windows-msvc") {
        println!("cargo:rustc-link-search=native={draco_install}");
    } else {
        println!("cargo:rustc-link-search=native={draco_install}/lib");
    }
    println!("cargo:rustc-link-lib=static=draco");

    println!("cargo:rerun-if-changed=cpp/decoder_api.cc");
    println!("cargo:rerun-if-changed=include/decoder_api.h");
    println!("cargo:rerun-if-changed=src/ffi.rs");
}
