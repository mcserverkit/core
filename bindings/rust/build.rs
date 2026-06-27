use std::process::Command;

fn main() {
    println!("Hello World!");

    // https://stackoverflow.com/questions/43292357/how-can-one-detect-the-os-type-using-rust
    // https://doc.rust-lang.org/std/process/struct.Command.html
    let _output = if cfg!(target_os = "windows") {
        Command::new("go")
    } else if cfg!(target_os = "linux") {
        Command::new("go")
    } else if cfg!(target_os = "macos") {
        Command::new("go")
    } else {
        println!("Operating system not supported");
        return;
    };
}
