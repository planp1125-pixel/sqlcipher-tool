fn main() {
    // Force bundled SQLCipher on all platforms
    println!("cargo:rustc-env=LIBSQLITE3_SYS_USE_PKG_CONFIG=0");
    println!("cargo:rustc-env=SQLCIPHER_BUNDLED=1");
    
    // Windows-specific configuration
    #[cfg(target_os = "windows")]
    {
        // Set OPENSSL_DIR if not already set (required by libsqlite3-sys)
        if std::env::var("OPENSSL_DIR").is_err() {
            println!("cargo:rustc-env=OPENSSL_DIR=C:\\openssl");
        }
        
        println!("cargo:rustc-env=OPENSSL_STATIC=1");
        println!("cargo:rustc-env=OPENSSL_VENDORED=1");
        println!("cargo:rustc-link-lib=static=crypt32");
        println!("cargo:rustc-link-lib=static=secur32");
        println!("cargo:rustc-link-lib=static=ncrypt");
        println!("cargo:rustc-link-lib=static=userenv");
        println!("cargo:rustc-link-lib=static=ws2_32");
    }
    
    tauri_build::build()
}
