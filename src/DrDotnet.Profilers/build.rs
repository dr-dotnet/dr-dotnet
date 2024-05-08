fn main() {

    if std::env::var("INCLUDE_UNRELEASED").unwrap_or_default() == "1" {
        println!("cargo:rustc-cfg=feature=\"include_unreleased\"");
    }

    if let Ok(val) = std::env::var("VERSION") {
        println!("cargo:rustc-env=CARGO_PKG_VERSION={}", val);
    }

    protobuf_codegen::Codegen::new()
        .pure()
        .include("protos")
        .input("protos/interop.proto")
        .cargo_out_dir("rust_protobuf_protos")
        .run_from_script();
}
