fn main() {
    println!("Running build.rs");
    println!("env VERSION={:?}", std::env::var("VERSION"));
    
    if let Ok(val) = std::env::var("VERSION") {
        println!("cargo:rustc-env=CARGO_PKG_VERSION={}", val);
    }
    
    //println!("cargo:rerun-if-env-changed=BUILD_VERSION");
}