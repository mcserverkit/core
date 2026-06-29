fn main() {
    println!("Hello World!");

    // https://stackoverflow.com/questions/43292357/how-can-one-detect-the-os-type-using-rust
    let _output = if cfg!(target_os = "windows") {
        let filename = "mcserverkit.windows-x86_64.zip";
    } else if cfg!(target_os = "linux") {
        let filename = "mcserverkit.linux-x86_64.zip";
    } else if cfg!(target_os = "macos") {
        let filename = "mcserverkit.macos-x86_64.zip";
    } else {
        println!("Operating system not supported");
        return;
    };
}
