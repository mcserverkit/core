fn main() {
    let architecture = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "arm64"
    } else {
        return;
    };
    // https://stackoverflow.com/questions/43292357/how-can-one-detect-the-os-type-using-rust
    let filename = if cfg!(target_os = "windows") {
        format!("mcserverkit.windows-{architecture}.zip")
    } else if cfg!(target_os = "linux") {
        format!("mcserverkit.linux-{architecture}.zip")
    } else if cfg!(target_os = "macos") {
        format!("mcserverkit.macos-{architecture}.zip")
    } else {
        println!("Operating system not supported");
        return;
    };

    println!("{filename}");
}
