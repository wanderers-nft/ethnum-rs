//! Allocation-free `core::fmt::Write` implementation.

// TODO(nlordell): replace with `MaybeUninit` slice methods once they stabilize.

use core::{
    fmt::{self, Write},
    mem::{self, MaybeUninit},
    str,
};

/// A format writer to a slice of uninitialized bytes.
pub struct Buffer<'a> {
    buf: &'a mut [MaybeUninit<u8>],
    pos: usize,
}

impl<'a> Buffer<'a> {
    /// Create a new empty buffer that can be written to.
    pub fn new(buf: &'a mut [MaybeUninit<u8>]) -> Self {
        Self { buf, pos: 0 }
    }

    /// Returns a string slice for the formatted data.
    pub fn as_str(&self) -> &str {
        // SAFETY: The buffer is only ever written with valid UTF-8 strings and
        // guaranteed to be initialized until `self.pos`.
        unsafe {
            let bytes = &*(&self.buf[..self.pos] as *const _ as *const [u8]);
            str::from_utf8_unchecked(bytes)
        }
    }
}

impl Write for Buffer<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let end = s.len().checked_add(self.pos).ok_or(fmt::Error)?;
        if end > self.buf.len() {
            return Err(fmt::Error);
        }

        // SAFETY: &[u8] and &[MaybeUninit<u8>] have the same layout
        let s: &[MaybeUninit<u8>] = unsafe { mem::transmute(s.as_bytes()) };

        #[cfg(not(debug_assertions))]
        self.buf[self.pos..end].copy_from_slice(s);
        #[cfg(debug_assertions)]
        unsafe {
            core::ptr::copy_nonoverlapping(
                s.as_ptr(),
                self.buf.get_unchecked_mut(self.pos..end).as_mut_ptr(),
                s.len(),
            )
        };

        self.pos = end;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_to_buffer() {
        let mut bytes = [MaybeUninit::uninit(); 12];
        let mut buf = Buffer::new(&mut bytes);
        buf.write_str("hello ").unwrap();
        buf.write_str("world!").unwrap();
        assert_eq!(buf.as_str(), "hello world!");
    }

    #[test]
    fn errors_on_overflow() {
        let mut bytes = [MaybeUninit::uninit(); 1];
        let mut buf = Buffer::new(&mut bytes);
        assert!(buf.write_str("too long!").is_err());
    }
}
