use crate::{
    error::{BCLError, Result},
    ins::{reg::InstructionRegistry, Instruction},
    opcode::OpCode,
};
use core::fmt::{self, Display};
use std::{mem, ops::Deref, sync::Arc};

//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Code<'a>(pub &'a [u8]);

#[derive(Debug, Clone, Copy)]
pub struct CodeIter<'a> {
    code: Code<'a>,
    ip: usize,
    reg: &'a InstructionRegistry,
}

#[derive(Debug, Clone, Copy)]
pub struct Disassembly<'a> {
    code: Code<'a>,
    reg: &'a InstructionRegistry,
}

//

impl<'a> Code<'a> {
    pub fn next(
        &self,
        reg: &'a InstructionRegistry,
        ip: &mut usize,
    ) -> Result<Arc<dyn Instruction>> {
        let opcode = OpCode::parse(*ip, self)?;
        *ip += mem::size_of::<OpCode>();
        let ins = reg
            .get(opcode)
            .ok_or(BCLError::InvalidInstructionOpcode(opcode))?;
        // println!("At opcode: {opcode} ip: {ip} ins: {}", ins.name());
        Ok(ins)
    }

    pub fn into_iter(self, reg: &'a InstructionRegistry) -> CodeIter<'a> {
        CodeIter {
            code: self,
            ip: 0,
            reg,
        }
    }

    pub fn disassemble(self, reg: &'a InstructionRegistry) -> Disassembly {
        Disassembly { code: self, reg }
    }
}

impl<'a> Deref for Code<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> Display for Disassembly<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.code.into_iter(self.reg);

        loop {
            let ip = iter.ip;
            let ins = if let Some(ins) = iter.next() {
                ins
            } else {
                break;
            };
            writeln!(f, "{ip:#016x} : {ins:?}")?;
        }

        Ok(())
    }
}

impl<'a> Iterator for CodeIter<'a> {
    type Item = Arc<dyn Instruction>;

    fn next(&mut self) -> Option<Self::Item> {
        let ins = self.code.next(self.reg, &mut self.ip).ok()?;
        ins.skip(&mut self.ip);
        Some(ins)
    }
}
