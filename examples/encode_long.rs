extern crate x86asm;

use std::io::Cursor;
use x86asm::{Instruction, InstructionWriter, Mnemonic, Mode, Operand, OperandSize, Reg, RegScale};

fn main() {
    let buffer = Cursor::new(Vec::new());
    let mut writer = InstructionWriter::new(buffer, Mode::Long); 

    // mov rax, qword ptr [rip+100]
    // mov rbx, 500
    // sub rax, rbx
    // mov [rcx+rdx*4], rax
    
    let instructions = &[
        Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::RAX), Operand::IndirectDisplaced(Reg::RIP, 100, Some(OperandSize::Qword), None)),
        Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::RBX), Operand::Literal32(500)),
        Instruction::new2(Mnemonic::SUB, Operand::Direct(Reg::RAX), Operand::Direct(Reg::RBX)),
        Instruction::new2(Mnemonic::MOV, Operand::IndirectScaledIndexed(Reg::RCX, Reg::RDX, RegScale::Four, Some(OperandSize::Qword), None), Operand::Direct(Reg::RAX)),
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
