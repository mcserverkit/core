use std::ffi;

unsafe extern "C" {
    unsafe fn Install(version: *const ffi::c_char) -> *const ffi::c_char;
    unsafe fn Create(name: *const ffi::c_char, eula: bool) -> *const ffi::c_char;
    unsafe fn Start(name: *const ffi::c_char, memory: *const ffi::c_char) -> *const ffi::c_char;
}

pub fn install(version: &str) -> Option<String> {
    let version: ffi::CString = ffi::CString::new(version).expect("");
    unsafe {
        let err: *const i8 = Install(version.as_ptr());
        if err.is_null() {
            None
        } else {
            Some(ffi::CStr::from_ptr(err).to_str().unwrap().to_string())
        }
    }
}

pub fn create(name: &str, eula: bool) -> Option<String> {
    let name: ffi::CString = ffi::CString::new(name).expect("");
    unsafe {
        let err: *const i8 = Create(name.as_ptr(), eula);
        if err.is_null() {
            None
        } else {
            Some(ffi::CStr::from_ptr(err).to_str().unwrap().to_string())
        }
    }
}

pub fn start(name: &str, memory: &str) -> Option<String> {
    let name: ffi::CString = ffi::CString::new(name).expect("");
    let memory: ffi::CString = ffi::CString::new(memory).expect("");
    unsafe {
        let err: *const i8 = Start(name.as_ptr(), memory.as_ptr());
        if err.is_null() {
            None
        } else {
            Some(ffi::CStr::from_ptr(err).to_str().unwrap().to_string())
        }
    }
}
