use std::ffi::c_void;
use std::ptr;

type LPCWSTR = *const u16;
type DWORD = i32;
type HMODULE = c_void;

#[link(name = "kernel32")]
#[no_mangle]
extern "system" {
    fn LoadLibraryExW(
        name: LPCWSTR,
        file_handle: c_void,
        params: DWORD,
    ) -> HMODULE;
}

struct DYL {
    bytes: usize,
}

impl DYL {
    pub fn new(_name: String) -> Self {
        return DYL { bytes: 3 };
    }

    /// Opens the actual file
    fn open() {}
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_some_object() {
        let newdyl: DYL = DYL::new("Something".to_string());
        assert_ne!(newdyl.bytes, 0);
    }
}
