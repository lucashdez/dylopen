use std::ffi::c_void;
use std::ptr;

type LPCWSTR = *const u16;
type DWORD = i32;
type HMODULE = *const c_void;

#[link(name = "kernel32")]
extern "system" {
    fn LoadLibraryExW(
        name: LPCWSTR,
        file_handle: HMODULE,
        params: DWORD,
    ) -> HMODULE;
}

pub fn load_library() -> Result<HMODULE, ()> {}
