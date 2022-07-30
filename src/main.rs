use code::Code;
use error::BCLError;
use std::process::ExitCode;
use vm::VirtualMachine;

//

pub mod code;
pub mod error;
pub mod ins;
pub mod opcode;
pub mod stack;
pub mod vm;

//

fn main() -> ExitCode {
    let code = Code(&[
        1, 0, // dbg
        2, 0, // prnt_inc
        100, 0, 2, // jmp 2
        0, 0, 2, // exit 2
    ]);
    // let code = Code(b"\x01\x00\x02\x00\x64\x00\x01\x00\x00\x02");
    // let code = Code(b"\x64\x00\x01");

    let mut vm = VirtualMachine::default();

    println!("Running:\n{}", code.disassemble(vm.registry()));
    // panic!();

    match vm.run(code) {
        Ok(..) => unreachable!(),
        Err(BCLError::Exit(code)) => code,
        Err(err) => {
            eprintln!("BCL VM Error: {err}");
            ExitCode::FAILURE
        }
    }
}
