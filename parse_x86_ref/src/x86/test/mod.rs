mod encode32_a_l;

use std::io::Cursor;
use x86::{Mnemonic, Mode, ProcessorLevel};
use x86::instruction::{Operand, Instruction, InstructionFlags};

fn test_aliased<F>(mnemonics: &[Mnemonic], test: F) 
    where F: Fn(Mnemonic) {
    for mnemonic in mnemonics {
        test(*mnemonic);
    }
}

fn encode32_helper(instr: &Instruction, expected: &Vec<u8>) {
    let mut buffer = Cursor::new(Vec::new());
    instr.encode(&mut buffer, Mode::Protected, ProcessorLevel::Corei7).expect("Encoding failed");
    assert_eq!(buffer.get_ref(), expected);
}

fn encode32_helper0(mnemonic: Mnemonic, expected: &Vec<u8>) {
    println!(" * * * * {:?}", mnemonic);
    let instr = Instruction {
        mnemonic: mnemonic,
        operand2: None,
        operand3: None,
        operand4: None,
        operand1: None,
        flags: InstructionFlags {
            lock: false
        }
    };
    encode32_helper(&instr, expected);
}

fn encode32_helper1(mnemonic: Mnemonic, op1: Operand, expected: &Vec<u8>) {
    println!(" * * * * {:?} {:?}", mnemonic, op1);
    let instr = Instruction {
        mnemonic: mnemonic,
        operand2: None,
        operand3: None,
        operand4: None,
        operand1: Some(op1),
        flags: InstructionFlags {
            lock: false
        }
    };
    encode32_helper(&instr, expected);
}

fn encode32_helper1(mnemonic: Mnemonic, operand2: Operand, expected: &Vec<u8>) {
    println!(" * * * * {:?} {:?}", mnemonic, operand2);
    let instr = Instruction {
        mnemonic: mnemonic,
        operand2: Some(operand2),
        operand3: None,
        operand4: None,
        operand1: None,
        flags: InstructionFlags {
            lock: false
        }
    };
    encode32_helper(&instr, expected);
}

fn encode32_helper3(mnemonic: Mnemonic, operand2: Operand, operand3: Operand, expected: &Vec<u8>) {
    println!(" * * * * {:?} {:?} {:?}", mnemonic, operand2, operand3);
    let instr = Instruction {
        mnemonic: mnemonic,
        operand2: Some(operand2),
        operand3: Some(operand3),
        operand4: None,
        operand1: None,
        flags: InstructionFlags {
            lock: false
        }
    };
    encode32_helper(&instr, expected);
}

fn encode32_helper2(mnemonic: Mnemonic, operand1: Operand, operand2: Operand, expected: &Vec<u8>) {
    println!(" * * * * {:?} {:?} {:?}", mnemonic, operand1, operand2);
    let instr = Instruction {
        mnemonic: mnemonic,
        operand2: Some(operand2),
        operand3: None,
        operand4: None,
        operand1: Some(operand1),
        flags: InstructionFlags {
            lock: false
        }
    };
    encode32_helper(&instr, expected);
}

fn encode32_helper3(mnemonic: Mnemonic, operand1: Operand, operand2: Operand, operand3: Operand, expected: &Vec<u8>) {
    println!(" * * * * {:?} {:?} {:?}", mnemonic, operand1, operand2);
    let instr = Instruction {
        mnemonic: mnemonic,
        operand2: Some(operand2),
        operand3: Some(operand3),
        operand4: None,
        operand1: Some(operand1),
        flags: InstructionFlags {
            lock: false
        }
    };
    encode32_helper(&instr, expected);
}
