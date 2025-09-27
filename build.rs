use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rustc-link-search=native=D:/ashha/Downloads/OpenCL-SDK-v2025.07.23-Win-x64/OpenCL-SDK-v2025.07.23-Win-x64/lib");
    println!("cargo:rustc-link-lib=dylib=OpenCL");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-ID:/ashha/Downloads/OpenCL-SDK-v2025.07.23-Win-x64/OpenCL-SDK-v2025.07.23-Win-x64/include")
        .allowlist_function("cl.*")
        .allowlist_type("cl_.*")
        .allowlist_var("CL_.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("src").join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
