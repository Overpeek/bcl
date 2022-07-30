use crate::error::{BCLError, Result};
use core::fmt::{self, Display};

//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OpCode(pub u16);

//

impl OpCode {
    pub fn parse(ip: usize, code: &[u8]) -> Result<Self> {
        if let &[a, b] = code.get(ip..=ip + 1).ok_or(BCLError::OutOfCode)? {
            Ok(Self(u16::from_le_bytes([a, b])))
        } else {
            unreachable!()
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}
