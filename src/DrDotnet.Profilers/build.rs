fn main() {
    // For debugging purpose:
    //panic!("[build.rs] VERSION={:?}", std::env::var("VERSION"));
    
    if let Ok(val) = std::env::var("VERSION") {
        println!("cargo:rustc-env=CARGO_PKG_VERSION={}", val);
    }
}