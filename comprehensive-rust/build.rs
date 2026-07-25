// build.rs
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // --- rerun-if-changed directives ---
    println!("cargo:rerun-if-changed=c_src/c_init.c");
    println!("cargo:rerun-if-changed=c_src/text_analysis.c");
    println!("cargo:rerun-if-changed=c_src/text_analysis.h");
    println!("cargo:rerun-if-changed=cpp_src/interner.cpp");
    println!("cargo:rerun-if-changed=cpp_src/interner.h");
    println!("cargo:rerun-if-changed=cpp_src/interner.hpp");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("OUT_DIR = {out_dir:?}");

    let c_src_dir = PathBuf::from("c_src");
    let cpp_src_dir = PathBuf::from("cpp_src");

    let mut objects = Vec::new();

    // 1. 编译 C 文件
    let c_files = ["c_init.c", "text_analysis.c"];
    for file in &c_files {
        let obj_file = out_dir.join(file).with_extension("o");
        let status = Command::new("cc")
            .args(&[
                "-c",
                c_src_dir.join(file).to_str().unwrap(),
                "-o",
                obj_file.to_str().unwrap(),
            ])
            .status()
            .expect("Failed to compile C code");
        assert!(status.success());
        objects.push(obj_file);
    }

    // 2. 编译 C++ 文件 (interner.cpp)
    let cpp_file = "interner.cpp";
    let cpp_obj = out_dir.join(cpp_file).with_extension("o");
    let status = Command::new("c++")
        .args(&[
            "-c",
            "-std=c++17",                     // ✅ 改为 C++17
            "-I", cpp_src_dir.to_str().unwrap(),
            cpp_src_dir.join(cpp_file).to_str().unwrap(),
            "-o",
            cpp_obj.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to compile C++ code");
    assert!(status.success());
    objects.push(cpp_obj);

    // 3. 打包静态库
    let lib_file = out_dir.join("libcombined.a");
    let mut ar_args = vec!["rcs", lib_file.to_str().unwrap()];
    for obj in &objects {
        ar_args.push(obj.to_str().unwrap());
    }
    let status = Command::new("ar")
        .args(&ar_args)
        .status()
        .expect("Failed to create static library");
    assert!(status.success());

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=combined");

    // Pass the linker flag directly to ensure it's applied to ALL targets
    // (including integration tests that declare their own extern "C" functions).
    println!("cargo:rustc-link-arg=-lcombined");

    // 链接 C++ 标准库
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=c++");
    #[cfg(not(target_os = "macos"))]
    println!("cargo:rustc-link-lib=stdc++");
}
