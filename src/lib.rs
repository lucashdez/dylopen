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
