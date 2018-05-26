mod raw;
mod atag;

pub use self::atag::*;

/// The address at which the firmware loads the ATAGS.
const ATAG_BASE: usize = 0x100;

/// An iterator over the ATAGS on this system.
pub struct Atags {
    ptr: &'static raw::Atag,
}

impl Atags {
    /// Returns an instance of `Atags`, an iterator over ATAGS on this system.
    pub fn get() -> Atags {
        Atags {
            ptr: unsafe { &*(ATAG_BASE as *const raw::Atag) }
        }
    }
    pub fn current(&self) -> Atag {
        Atag::from(self.ptr)
    }
}

impl Iterator for Atags {
    type Item = Atag;

    fn next(&mut self) -> Option<Atag> {
        match self.ptr.next() {
            Some(raw_atag) => {
                self.ptr = raw_atag;
                Some(Atag::from(raw_atag))
            },
            _ => None,
        }
    }
}
