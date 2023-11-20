fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if std::env::var("TARGET").unwrap() == "aarch64-linux-android" {
        println!("cargo:rustc-link-search=runtime_libs/arm64-v8a");
    }
}
