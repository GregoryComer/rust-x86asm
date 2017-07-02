extern crate x86asm;

use std::io::Cursor;
use x86asm::{Instruction, InstructionWriter, Mnemonic, Mode, Operand, OperandSize, Reg, RegScale};

fn main() {
    let buffer = Cursor::new(Vec::new());
    let mut writer = InstructionWriter::new(buffer, Mode::Real); 

    // mov ax, [bx+si]
    // add ax, bx
    // mov [bp+si], ax
    
    let instructions = &[
        Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::AX), Operand::IndirectScaledIndexed(Reg::BX, Reg::SI, RegScale::One, Some(OperandSize::Word), None)), // mov ax, [bx+si]
        Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::Direct(Reg::BX)), // add ax, bx
        Instruction::new2(Mnemonic::MOV, Operand::IndirectScaledIndexed(Reg::BX, Reg::SI, RegScale::One, Some(OperandSize::Word), None), Operand::Direct(Reg::AX)), // mov [bp+si]
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
