use super::{dbg::Dbg, exit::Exit, jmp::Jmp, print_inc::PrintInc, Instruction, InstructionBuilder};
use crate::{
    error::{BCLError, Result},
    opcode::OpCode,
};
use core::fmt::{self, Debug};
use std::sync::Arc;

//

#[derive(Clone)]
pub struct InstructionRegistry {
    instructions: Vec<Option<Arc<dyn Instruction>>>,
}

//

impl Debug for InstructionRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InstructionRegistry")
            .field(
                "instructions",
                &self
                    .instructions
                    .iter()
                    .map(|s| s.as_ref())
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

impl Default for InstructionRegistry {
    fn default() -> Self {
        let mut reg = Self {
            instructions: Default::default(),
        };

        reg.add_ins::<Exit>().unwrap();
        reg.add_ins::<Dbg>().unwrap();
        reg.add_ins::<PrintInc>().unwrap();

        reg.add_ins::<Jmp>().unwrap();

        reg
    }
}

//

impl InstructionRegistry {
    pub fn get(&self, opcode: OpCode) -> Option<Arc<dyn Instruction>> {
        self.instructions
            .get(opcode.0 as usize)
            .and_then(Option::as_ref)
            .map(Arc::clone)
    }

    pub fn run(&self, opcode: OpCode, ip: &mut usize, code: &[u8], stack: &mut [u8]) -> Result<()> {
        let ins = self
            .get(opcode)
            .ok_or(BCLError::InvalidInstructionOpcode(opcode))?;
        ins.exec(ip, code, stack)
    }

    pub fn add_ins<T: Instruction + InstructionBuilder + Default + 'static>(
        &mut self,
    ) -> Result<()> {
        let opcode = T::OPCODE as usize;
        if self.instructions.len() < opcode + 1 {
            self.instructions.resize_with(opcode + 1, || None);
        }

        let new = T::default();

        let ins = &mut self.instructions[opcode];
        match ins {
            Some(old) => Err(BCLError::InstructionConflict {
                a: old.name().into(),
                b: new.name().into(),
            }),
            _ => {
                *ins = Some(Arc::new(new));
                Ok(())
            }
        }
    }
}
