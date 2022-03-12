fn main() {
    let out_file = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("wasm.rs");
    let header =
        std::path::PathBuf::from("/Volumes/Dev/secondstate/me/wasm-rust/wasm-c-api/include/wasm.h");
    bindgen::builder()
        .header(
            header
                .to_str()
                .unwrap_or_else(|| panic!("`{}` must be a utf-8 path", header.display())),
        )
        .prepend_enum_name(false) // The API already prepends the name.
        .generate()
        .expect("failed to generate bindings")
        .write_to_file(out_file)
        .expect("failed to write bindings");

    println!("cargo:rustc-link-lib=dylib=wasmedge_c");
}
