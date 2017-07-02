extern crate x86asm;

use std::io::Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

fn main() {
    let buffer = Cursor::new(Vec::new());
    let mut writer = InstructionWriter::new(buffer, Mode::Protected); 
    
    let bytes_written = 
        writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
        writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
        writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx

    print!("Output ({} bytes): ", bytes_written);
    for byte in writer.get_inner_writer_ref().get_ref().iter() {
        print!("{:02X} ", byte);
    }
    println!("");
}
