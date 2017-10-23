use ::{Instruction, Mode, OperandSize};
use ::test::instruction_tests::{INSTRUCTION_TESTS, TestCase};

use std::io::Cursor;

#[test]
fn run_instruction_tests() {
    for case in INSTRUCTION_TESTS.iter() {
        let mut buffer = Cursor::new(Vec::new());
        println!("");
        println!("Instr: {:?}", case.instruction);

        case.instruction.encode(&mut buffer, addr_size_to_mode(case.addr_size))
            .expect("Encoding failed");

        if &buffer.get_ref()[..] != case.expected {
            println!("Test Failed.");
            print!("Output:   ");
            write_hex_slice(buffer.get_ref());
            println!("");
            print!("Expected: ");
            write_hex_slice(case.expected);
            println!("");
            println!("Instruction: {:?}", case.instruction);
            println!("Address Size: {:?}", case.addr_size);
            panic!("Failure.");
        } else {
            println!("Test Passed: {:?}", case.instruction);
        }
    }
}

fn addr_size_to_mode(addr_size: OperandSize) -> Mode {
    match addr_size {
        OperandSize::Word => Mode::Real,
        OperandSize::Dword => Mode::Protected,
        OperandSize::Qword => Mode::Long,
        _ => panic!("Invalid addressing size {:?}.", addr_size)
    }
}

fn write_hex_slice(slice: &[u8]) {
    for b in slice {
        print!("{:02X} ", b);
    }
}
