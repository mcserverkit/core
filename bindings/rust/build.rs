// https://stackoverflow.com/questions/43292357/how-can-one-detect-the-os-type-using-rust
fn main() {
    println!("Hello World!");

    if cfg!(target_os = "windows") {
    } else if cfg!(target_os = "linux") {
    } else if cfg!(target_os = "macos") {
    }
}
