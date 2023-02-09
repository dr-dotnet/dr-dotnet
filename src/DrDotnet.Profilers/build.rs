

fn main() {
    // For debugging purpose:
    //panic!("[build.rs] VERSION={:?}", std::env::var("VERSION"));
    
    if let Ok(val) = std::env::var("VERSION") {
        println!("cargo:rustc-env=CARGO_PKG_VERSION={}", val);
    }

    protobuf_codegen::Codegen::new()
        .pure()
        .include("protos")
        .input("protos/triangle.proto")
        .cargo_out_dir("rust_protobuf_protos")
        .run_from_script();
}