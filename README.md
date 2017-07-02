# x86asm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.sm - A Rust crate for assembling & disassembling x86/64 instructions

## About
The x86asm crate is a Rust library designed for programatically assembling & disassembling x86 instructions, supporting real, protected, and long mode assembly.

## Status
This project is currently in active development. The core instruction encoding logic is complete and all core instructions are implemented, but many instruction extensions have only partial support.

## Instruction Set Support
 * Core Instructions - Full Support
 * SSE1/2 - Full Support
 * SSE3+ - Partial Support
 * AVX1/2 - Partial Support
 * AVX512 - Partial Support
 * BMI - Partial Support
 * MPX - Full Support

## Usage Examples
See the *examples* directory for full examples.

Encode a series of instructions to an in-memory buffer:
```rust
use std::io:Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};

...

let buffer = Cursor::new(Vec::new());
let mut writer = InstructionWriter::new(buffer, Mode::Protected);

let bytes_written = 
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EAX), Operand::Literal32(10)).unwrap() + // mov eax, 10
    writer.write2(Mnemonic::MOV, Operand::Direct(Reg::EBX), Operand::Literal32(20)).unwrap() + // mov ebx, 20
    writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Direct(Reg::EBX)).unwrap(); // add eax, ebx
```

A more in-depth example demonstrating different addressing modes.
```rust
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
```

## Build
The x86asm library uses Cargo. To build, clone the repository using Git and run `cargo build` from the command line.

## Contribute
Contributions are welcome! If you're not inclined to dig into the code yourself but encounter an issue, feel free to submit an issue using the GitHub issue tracker. Before contributing, please see *parse_x86_ref/README* and *src/test/README* for a detailed description of the instruction format, as well as in-depth guidelines for contributing.

## Major Open Tasks
* **Exhaustive Instruction Definitions** - Instructions starting with A-O, as well as most instructions starting with P have full definitions. Instructions starting with Q-Z not included in the core instruction set, or included in SSE1/2 need to be defined. See *parse_x86_ref/README* for an in-depth description of the instruction encoding format and contribution process. Each new instruction implemented should have corresponding regression tests in the test module. See *src/test/README* for a description of the test format and contribution process.
* **Improved Test Coverage** - Real and long mode instruction encoding currently have a basic set of regression tests, covering addressing modes and instruction formats, but lack the level comprehensive test coverage that protected mode encoding has. In addition, instruction dissassembly test coverage is poor and should be improved.
* **Instruction Description Format** - The current instruction definition format is based on the ref.x86asm.net instruction definition format, with a number of modifications made to fascilitate AVX and other instruction set extensions. However, this instruction format, with the number of extensions made to it, has become somewhat bloated and adding support for new instructions is more difficult than it needs to be. Long term, moving to a more compact instruction encoding format would be ideal. See *parse_x86_ref/README* for more detail.

## Contributors
* Gregory Comer - Author

## License
This project is open source and is licensed under the MIT License. The full text is included in the repository.
