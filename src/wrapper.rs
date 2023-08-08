use std::ffi::c_void;
use std::ptr;

type LPCWSTR = *const u16;
type DWORD = i32;
type HMODULE = *const c_void;

#[derive(Debug)]
pub enum WrapperError {
    NullPointer(String),
}

#[link(name = "kernel32")]
extern "system" {
    fn LoadLibraryExW(
        name: LPCWSTR,
        file_handle: HMODULE,
        params: DWORD,
    ) -> HMODULE;
}

fn to_null_terminated_u16(s: &str) -> *const u16 {
    s.encode_utf16()
        .chain(Some(0))
        .collect::<Vec<u16>>()
        .as_ptr()
}

pub fn load_library(name: &str) -> Result<HMODULE, WrapperError> {
    let nameu16: *const u16 = to_null_terminated_u16(name);
    let result = unsafe { LoadLibraryExW(nameu16, ptr::null(), 0) };

    if result == ptr::null() {
        return Err(WrapperError::NullPointer("Couldn't open the library. The library is not in the specified path or it doesn't exist".to_owned()));
    }
    return Ok(result);
}
