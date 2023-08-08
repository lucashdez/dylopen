use std::ffi::c_void;

mod wrapper;

struct DYL {
    bytes: usize,
    handle: *const c_void,
}

impl DYL {
    pub fn new(_name: String) -> Self {
        return DYL {
            bytes: 3,
            handle: DYL::open(_name),
        };
    }

    /// Opens the actual file
    fn open(name: String) -> HMODULE {
        unsafe {
            LoadLibraryExW(
                name.encode_utf16().collect::<Vec<u16>>().as_ptr(),
                ptr::null(),
                0,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_some_object() {
        let newdyl: DYL = DYL::new("Something".to_string());
    }

    #[test]
    fn opens_something() {}
}
