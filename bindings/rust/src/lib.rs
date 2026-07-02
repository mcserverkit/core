use std::ffi;

unsafe extern "C" {
    fn Install(version: *const ffi::c_char);
    fn Create(name: *const ffi::c_char, eula: bool);
    fn Start(name: *const ffi::c_char, memory: *const ffi::c_char);
}

pub fn install() {
    unsafe { Install() }
}

pub fn create() {
    unsafe { Create() }
}

pub fn start() {
    unsafe { Start() }
}
