extern crate x86asm;

use std::io::Cursor;
use x86asm::{Instruction, InstructionWriter, Mnemonic, Mode, Operand, OperandSize, Reg};

fn main() {
    let buffer = Cursor::new(Vec::new());
    let mut writer = InstructionWriter::new(buffer, Mode::Protected); 

    let instructions = &[
        Instruction::new1(Mnemonic::PUSH, Operand::Direct(Reg::EBP)),
        Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::EBP), Operand::Direct(Reg::ESP)),
        Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::IndirectDisplaced(Reg::EBP, 12, Some(OperandSize::Dword), None)),
        Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::IndirectDisplaced(Reg::EBP, 8, Some(OperandSize::Dword), None)),
        Instruction::new0(Mnemonic::LEAVE),
        Instruction::new0(Mnemonic::RET),
    ];

    let mut bytes_written = 0;

    for instr in instructions {
        bytes_written += writer.write(instr).unwrap();
    }
    
    print!("Output ({} bytes): ", bytes_written);
    for byte in writer.get_inner_writer_ref().get_ref().iter() {
        print!("{:02X} ", byte);
    }
    println!("");
}
