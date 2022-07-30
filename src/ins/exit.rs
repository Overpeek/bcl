use super::{Instruction, InstructionBuilder};
use crate::error::{BCLError, Result};
use std::process::ExitCode;

//

#[derive(Debug, Clone, Copy, Default)]
pub struct Exit;

//

impl Instruction for Exit {
    fn name(&self) -> &'static str {
        "exit"
    }

    fn len(&self) -> usize {
        1
    }

    fn exec(&self, ip: &mut usize, code: &[u8], _: &mut [u8]) -> Result<()> {
        let exit_code = code[*ip];
        Err(BCLError::Exit(ExitCode::from(exit_code)))
    }
}

impl InstructionBuilder for Exit {
    const OPCODE: u16 = 0;
}
