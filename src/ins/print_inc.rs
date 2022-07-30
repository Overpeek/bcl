use super::{Instruction, InstructionBuilder};
use crate::error::Result;
use std::sync::atomic::{AtomicUsize, Ordering};

//

#[derive(Debug, Clone, Copy, Default)]
pub struct PrintInc;

//

pub static COUNTER: AtomicUsize = AtomicUsize::new(0);

//

impl Instruction for PrintInc {
    fn name(&self) -> &'static str {
        "prnt_inc"
    }

    fn exec(&self, _: &mut usize, _: &[u8], _: &mut [u8]) -> Result<()> {
        println!(
            "PRINT INC INSTRUCTION {}",
            COUNTER.fetch_add(1, Ordering::SeqCst)
        );
        Ok(())
    }
}

impl InstructionBuilder for PrintInc {
    const OPCODE: u16 = 2;
}
