use super::{Instruction, InstructionBuilder};
use crate::error::Result;

//

#[derive(Debug, Clone, Copy, Default)]
pub struct Jmp;

//

impl Instruction for Jmp {
    fn name(&self) -> &'static str {
        "jmp"
    }

    fn len(&self) -> usize {
        1
    }

    fn exec(&self, ip: &mut usize, code: &[u8], _: &mut [u8]) -> Result<()> {
        // println!("jump from {} {:?}", *ip, code);
        *ip = code[*ip] as usize;
        // println!("jump to {}", *ip);
        Ok(())
    }
}

impl InstructionBuilder for Jmp {
    const OPCODE: u16 = 100;
}
