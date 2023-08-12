#[warn(dead_code)]
use std::ffi::c_void;
use std::ptr;

#[derive(Debug)]
pub enum DylopenErrors {
    WrapperError(u32),
}

#[link(name = "kernel32")]
extern "stdcall" {
    /// .
    // TODO : Some explanation of this
    fn LoadLibraryExW(
        name: *const u16,
        file_handle: *const c_void,
        params: u32,
    ) -> *const c_void;

    //  NOTE : This return value is a boolean
    fn FreeLibrary(lib_handle: *const c_void) -> i32;

    // NOTE : Gets the error code of the last error ocurred in the call to the
    // winapi.
    fn GetLastError() -> u32;

}

/// The safe wrapper for GetLastError.
/// Returns the calling thread's last error-code.
fn s_get_last_error() -> u32 {
    unsafe { GetLastError() }
}

fn to_null_terminated_u16(s: &str) -> *const u16 {
    s.encode_utf16()
        .chain(Some(0))
        .collect::<Vec<u16>>()
        .as_ptr()
}

/// TODO: Write docs for load_library
pub fn load_library(name: &str) -> Result<*const c_void, DylopenErrors> {
    let nameu16: *const u16 = to_null_terminated_u16(name);
    let result = unsafe { LoadLibraryExW(nameu16, ptr::null(), 0) };

    if result == ptr::null() {
        return Err(DylopenErrors::WrapperError(s_get_last_error()));
    }
    return Ok(result);
}

/// TODO: Write docs for close_library
pub fn close_library(lib_handle: *const c_void) -> Result<(), DylopenErrors> {
    let result = unsafe { FreeLibrary(lib_handle) };
    if result == 0 {
        let err = unsafe { GetLastError() };
        return Err(DylopenErrors::WrapperError(s_get_last_error()));
    }
    Ok(())
}
