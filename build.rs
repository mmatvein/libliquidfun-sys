extern crate cmake;

use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=static=box2d");
    println!("cargo:rerun-if-changed=box2d/");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=include/");

    let box2d_path = build_box2d();
    let box2d_include_path = box2d_path.join(std::path::PathBuf::from("include"));
    cc::Build::new()
        .cpp(true)
        .includes([&box2d_include_path, &std::path::PathBuf::from("include")])
        .file("src/extras.cpp")
        .compile("libliquidfun-sys-extras.a");
    generate_bindings(box2d_include_path);
}

fn build_box2d() -> PathBuf {
    let box2d_path = cmake::Config::new("box2d")
        .define("BOX2D_BUILD_UNIT_TESTS", "OFF")
        .define("BOX2D_BUILD_TESTBED", "OFF")
        .define("BOX2D_BUILD_DOCS", "OFF")
        .define("BOX2D_USER_SETTINGS", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .build();

    println!(
        "cargo:rustc-link-search=native={}/lib",
        box2d_path.display()
    );
    println!("cargo:include={}/include", box2d_path.display());

    return box2d_path;
}

fn generate_bindings(box2d_include_path: PathBuf) {
    let include_path = std::path::PathBuf::from("include");
    let mut autocxx_build =
        autocxx_build::Builder::new("src/lib.rs", [&box2d_include_path, &include_path])
            .build()
            .unwrap();
    autocxx_build
        .include(box2d_include_path)
        .include(include_path)
        .flag_if_supported("-std=c++14")
        .compile("libliquidfun-sys.a");
}
