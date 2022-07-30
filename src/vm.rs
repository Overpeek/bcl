use crate::{code::Code, error::Result, ins::reg::InstructionRegistry, stack::Stack};

//

#[derive(Debug, Clone, Default)]
pub struct VirtualMachine {
    reg: InstructionRegistry,
    stack: Stack,
}

//

impl VirtualMachine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(reg: InstructionRegistry, stack: Stack) -> Self {
        Self { reg, stack }
    }

    pub fn run(&mut self, code: Code) -> Result<()> {
        let mut ip = 0;
        loop {
            self.run_ins(&mut ip, code)?;
        }
    }

    pub fn run_ins(&mut self, ip: &mut usize, code: Code) -> Result<()> {
        let ins = code.next(&self.reg, ip)?;
        ins.exec(ip, &code, &mut self.stack)?;
        Ok(())
    }

    pub fn registry(&self) -> &InstructionRegistry {
        &self.reg
    }
}
