# x86asm - A Rust crate for assembling & disassembling x86/64 instructions

The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions. This project supports real, protected, and long mode assembly.

## Status
This crate is feature complete but immature. If you encounter bugs or wish for a helpful new feature, feel free to contribute or create an issue. See the contributing  section below.

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

// mov eax, 10
// mov ebx, 20
// add eax, ebx

writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)); // mov eax, 10
writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)); // mov ebx, 20
writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes:
```rust
let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

// mov ebx, dword ptr [eax]
// mov ebx, dword ptr [eax+5]
// mov ebx, dword ptr [eax+ecx*2]
// mov ebx, dword ptr [eax+ecx*2+5]
// mov ebx, dword ptr ds:5

let instructions = &[
    Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Indirect(Reg::EAX, Some(OperandSize::Dword), None)), // mov ebx, dword ptr [eax]
    Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::IndirectDisplaced(Reg::EAX, 5, Some(OperandSize::Dword), None)), // mov ebx, dword ptr [eax+5]
    Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::IndirectScaledIndexed(Reg::EAX, Reg::ECX, RegScale::Two, Some(OperandSize::Dword), None)), // mov ebx, dword ptr [eax+ecx*2]
    Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::IndirectScaledIndexedDisplaced(Reg::EAX, Reg::ECX, RegScale::Two, 5, Some(OperandSize::Dword), None)), // mov ebx, dword ptr [eax+ecx*2+5]
    Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Memory(5, Some(OperandSize::Dword), None)), // mov ebx, dword ptr ds:5
];

for instr in instructions { writer.write(instr).unwrap(); }
```

Real mode assembly:
```rust
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

for instr in instructions { writer.write(instr).unwrap(); }
```

Long mode assembly:
```rust
let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Long);

// mov rax, qword ptr [rip+100]
// mov rbx, 500
// sub rax, rbx
// mov [rcx+rdx*4], rax

let instructions = &[
    Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::RAX), Operand::IndirectDisplaced(Reg::RIP, 100, Some(OperandSize::Qword), None)), // mov rax, qword ptr [rip+100]
    Instruction::new2(Mnemonic::MOV, Operand::Direct(Reg::RBX), Operand::Literal32(500)), // mov rbx, 500
    Instruction::new2(Mnemonic::SUB, Operand::Direct(Reg::RAX), Operand::Direct(Reg::RBX)), // sub rax, rbx
    Instruction::new2(Mnemonic::MOV, Operand::IndirectScaledIndexed(Reg::RCX, Reg::RDX, RegScale::Four, Some(OperandSize::Qword), None), Operand::Direct(Reg::RAX)), // mov [rcx+rdx*4], rax
];

for instr in instructions { writer.write(instr).unwrap(); }
```
## Build
The x86asm crate uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line using either stable or nightly Rust.
## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *gen_defs/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.
