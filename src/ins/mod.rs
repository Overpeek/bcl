use crate::error::Result;
use core::fmt::{self, Debug};
use std::any::type_name;

//

pub mod dbg;
pub mod exit;
pub mod jmp;
pub mod print_inc;
pub mod reg;

//

pub trait Instruction {
    fn name(&self) -> &'static str
    where
        Self: 'static,
    {
        type_name::<Self>()
    }

    fn len(&self) -> usize {
        0
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn skip(&self, ip: &mut usize) {
        *ip += self.len();
    }

    fn exec(&self, ip: &mut usize, code: &[u8], stack: &mut [u8]) -> Result<()>;
}

pub trait InstructionBuilder {
    /// OpCode must not conflict with any
    /// other active instruction's OpCode
    const OPCODE: u16;
}

//

impl Debug for dyn Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
