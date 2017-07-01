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

fn encode32_helper_empty(mnemonic: Mnemonic, expected: &Vec<u8>) {
    println!(" * * * * {:?}", mnemonic);
    let instr = Instruction {
        mnemonic: mnemonic,
        source: None,
        source2: None,
        source3: None,
        destination: None,
        flags: InstructionFlags {
            lock: false
        }
    };
    encode32_helper(&instr, expected);
}

fn encode32_helper_d(mnemonic: Mnemonic, dest: Operand, expected: &Vec<u8>) {
    println!(" * * * * {:?} {:?}", mnemonic, dest);
    let instr = Instruction {
        mnemonic: mnemonic,
        source: None,
        source2: None,
        source3: None,
        destination: Some(dest),
        flags: InstructionFlags {
            lock: false
        }
    };
    encode32_helper(&instr, expected);
}

fn encode32_helper_s(mnemonic: Mnemonic, source: Operand, expected: &Vec<u8>) {
    println!(" * * * * {:?} {:?}", mnemonic, source);
    let instr = Instruction {
        mnemonic: mnemonic,
        source: Some(source),
        source2: None,
        source3: None,
        destination: None,
        flags: InstructionFlags {
            lock: false
        }
    };
    encode32_helper(&instr, expected);
}

fn encode32_helper_ss(mnemonic: Mnemonic, source: Operand, source2: Operand, expected: &Vec<u8>) {
    println!(" * * * * {:?} {:?} {:?}", mnemonic, source, source2);
    let instr = Instruction {
        mnemonic: mnemonic,
        source: Some(source),
        source2: Some(source2),
        source3: None,
        destination: None,
        flags: InstructionFlags {
            lock: false
        }
    };
    encode32_helper(&instr, expected);
}

fn encode32_helper_ds(mnemonic: Mnemonic, destination: Operand, source: Operand, expected: &Vec<u8>) {
    println!(" * * * * {:?} {:?} {:?}", mnemonic, destination, source);
    let instr = Instruction {
        mnemonic: mnemonic,
        source: Some(source),
        source2: None,
        source3: None,
        destination: Some(destination),
        flags: InstructionFlags {
            lock: false
        }
    };
    encode32_helper(&instr, expected);
}

fn encode32_helper_dss(mnemonic: Mnemonic, destination: Operand, source: Operand, source2: Operand, expected: &Vec<u8>) {
    println!(" * * * * {:?} {:?} {:?}", mnemonic, destination, source);
    let instr = Instruction {
        mnemonic: mnemonic,
        source: Some(source),
        source2: Some(source2),
        source3: None,
        destination: Some(destination),
        flags: InstructionFlags {
            lock: false
        }
    };
    encode32_helper(&instr, expected);
}
