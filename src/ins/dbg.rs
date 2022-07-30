use super::{Instruction, InstructionBuilder};
use crate::error::Result;

//

#[derive(Debug, Clone, Copy, Default)]
pub struct Dbg;

//

impl Instruction for Dbg {
    fn name(&self) -> &'static str {
        "dbg"
    }

    fn exec(&self, ip: &mut usize, _: &[u8], _: &mut [u8]) -> Result<()> {
        println!("DEBUG INSTRUCTION\nIP = {ip}");
        Ok(())
    }
}

impl InstructionBuilder for Dbg {
    const OPCODE: u16 = 1;
}
