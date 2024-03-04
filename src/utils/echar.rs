use std::fmt::{self, Debug, Display, Formatter};

/// Stack-allocated char-like type, for use with errors
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EChar<const N: usize = 4> {
    buf: [u8; N],
    len: u8,
}
impl<const N: usize> EChar<N> {
    pub fn new(bytes: &[u8], len: u8) -> Option<Self> {
        (len <= (N as u8) && bytes.len() >= (len as usize)).then(|| {
            let mut buf = [0u8; N];
            buf[..(len as usize)].copy_from_slice(&bytes[..(len as usize)]);
            Self { buf, len }
        })
    }
    pub fn as_slice(&self) -> &[u8] {
        &self.buf[..(self.len as usize)]
    }
}
impl<const N: usize> Display for EChar<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(bstr::BStr::new(&self.buf[..(self.len as usize)]), f)
    }
}

/// Simple intermediate for index printer that doesn't allocate
pub struct IdxPrint(pub usize);
impl Display for IdxPrint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.0 == usize::MAX {
            f.write_str("unknown index")
        } else {
            write!(f, "byte {}", self.0)
        }
    }
}
