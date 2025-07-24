//! Build script for projectm-rs
//! Version: 0.1.12

fn main() {
    // Explicitly link to the system-installed ProjectM v3.x
    println!("cargo:rustc-link-search=native=/usr/lib");
    println!("cargo:rustc-link-lib=projectM");
    
    // Also link SDL2 which is needed by projectM
    println!("cargo:rustc-link-lib=SDL2");
    
    // Set the runtime library path to prioritize system libraries
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib");
    
    // Print out some info for debugging
    if let Ok(lib_path) = std::env::var("LD_LIBRARY_PATH") {
        println!("LD_LIBRARY_PATH: {}", lib_path);
    }
}