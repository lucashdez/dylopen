use crate::wrapper::*;
use std::ffi::c_void;

mod wrapper;

struct DYL {
    bytes: usize,
    handle: *const c_void,
}

impl DYL {
    pub fn new(name: &str) -> Self {
        return DYL {
            bytes: 3,
            handle: load_library(name).expect(
                "Could not load the library while initializing the DYL",
            ),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_some_object() {
        let newdyl: DYL = DYL::new("./target/debug/libdylopen.rlib");
        let mut res = 1;
        if newdyl.handle.is_null() {
            res = 0;
        }
        assert_eq!(1, res);
    }

    #[test]
    fn opens_something() {}
}
