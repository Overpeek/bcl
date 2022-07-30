use crate::opcode::OpCode;
use std::{borrow::Cow, process::ExitCode};
use thiserror::Error;

//

pub type Result<T> = core::result::Result<T, BCLError>;

//

#[derive(Debug, Error)]
pub enum BCLError {
    #[error("instruction registry opcode conflict with '{a}' and '{b}'")]
    InstructionConflict {
        a: Cow<'static, str>,
        b: Cow<'static, str>,
    },

    #[error("invalid instruction opcode '{0}'")]
    InvalidInstructionOpcode(OpCode),

    #[error("invalid instruction parameters")]
    InvalidInstructionParams(),

    #[error("instruction pointer out of bounds")]
    OutOfCode,

    #[error("graceful exit '{0:?}'")]
    Exit(ExitCode),
}
