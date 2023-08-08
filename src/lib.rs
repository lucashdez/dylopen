use crate::wrapper::*;
use std::ffi::c_void;

mod wrapper;

struct DYL {
    bytes: usize,
    handle: *const c_void,
}

impl DYL {
    pub fn new(_name: &str) -> Self {
        return DYL {
            bytes: 3,
            handle: DYL::open(_name),
        };
    }

    /// Opens the actual file
    fn open(name: &str) -> *const c_void {
        load_library(name)
            .expect(format!("Error in lib.rs. Line {}", line!()).as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_some_object() {
        let newdyl: DYL = DYL::new("./target/debug/libdylopen.rlib");
        dbg!("AAAAAAAAAAAAAAAAAAAA");
    }

    #[test]
    fn opens_something() {}
}
