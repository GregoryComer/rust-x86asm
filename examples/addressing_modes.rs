extern crate x86asm;

use std::io::Cursor;
use x86asm::{Instruction, InstructionWriter, Mnemonic, Mode, Operand, OperandSize, Reg, RegScale};

fn main() {
    let buffer = Cursor::new(Vec::new());
    let mut writer = InstructionWriter::new(buffer, Mode::Protected); 

    let instructions = &[
        Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None)), // mov ebx, dword ptr [eax]
        Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::IndirectDisplaced(Reg::EAX, 5, Some(OperandSize::Dword), None)), // mov ebx, dword ptr [eax+5]
        Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::IndirectScaledIndexed(Reg::EAX, Reg::ECX, RegScale::Two, Some(OperandSize::Dword), None)), // mov ebx, dword ptr [eax+ecx*2]
        Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::IndirectScaledIndexedDisplaced(Reg::EAX, Reg::ECX, RegScale::Two, 5, Some(OperandSize::Dword), None)), // mov ebx, dword ptr [eax+ecx*2+5]
        Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Memory(5, Some(OperandSize::Dword), None)), // mov ebx, dword ptr ds:5
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
