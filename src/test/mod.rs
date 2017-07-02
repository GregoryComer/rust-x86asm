mod addressing16;
mod addressing32;
mod addressing64;
mod instr32_a_l;
mod instr32_m_u;
mod size_inference;

use std::io::Cursor;
use ::{Mnemonic, Mode, Operand, ProcessorLevel, InstructionEncodingError};
use ::instruction::{Instruction, InstructionFlags};

fn test_aliased<F>(mnemonics: &[Mnemonic], test: F) 
    where F: Fn(Mnemonic) {
    for mnemonic in mnemonics {
        test(*mnemonic);
    }
}

fn encode16_helper(instr: &Instruction, expected: &Vec<u8>) {
    let mut buffer = Cursor::new(Vec::new());
    instr.encode(&mut buffer, Mode::Real, ProcessorLevel::Corei7).expect("Encoding failed");
    assert_eq!(buffer.get_ref(), expected);
}

fn encode16_helper2(mnemonic: Mnemonic, operand1: Operand, operand2: Operand, expected: &Vec<u8>) {
    println!(" * * * * {:?} {:?} {:?}", mnemonic, operand1, operand2);
    let instr = Instruction {
        mnemonic: mnemonic,
        operand1: Some(operand1),
        operand2: Some(operand2),
        operand3: None,
        operand4: None,
        flags: Default::default()
    };
    encode16_helper(&instr, expected);
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
        flags: Default::default()
    };
    encode32_helper(&instr, expected);
}

fn encode32_helper1(mnemonic: Mnemonic, op1: Operand, expected: &Vec<u8>) {
    println!(" * * * * {:?} {:?}", mnemonic, op1);
    let instr = Instruction {
        mnemonic: mnemonic,
        operand1: Some(op1),
        operand2: None,
        operand3: None,
        operand4: None,
        flags: Default::default()
    };
    encode32_helper(&instr, expected);
}
fn encode32_helper2(mnemonic: Mnemonic, operand1: Operand, operand2: Operand, expected: &Vec<u8>) {
    println!(" * * * * {:?} {:?} {:?}", mnemonic, operand1, operand2);
    let instr = Instruction {
        mnemonic: mnemonic,
        operand1: Some(operand1),
        operand2: Some(operand2),
        operand3: None,
        operand4: None,
        flags: Default::default()
    };
    encode32_helper(&instr, expected);
}

fn encode32_helper2_flags(mnemonic: Mnemonic, operand1: Operand, operand2: Operand, flags: InstructionFlags, expected: &Vec<u8>) {
    println!(" * * * * {:?} {:?} {:?} {:?}", mnemonic, operand1, operand2, flags);
    let instr = Instruction {
        mnemonic: mnemonic,
        operand1: Some(operand1),
        operand2: Some(operand2),
        operand3: None,
        operand4: None,
        flags: flags
    };
    encode32_helper(&instr, expected);
}

fn encode32_helper3(mnemonic: Mnemonic, operand1: Operand, operand2: Operand, operand3: Operand, expected: &Vec<u8>) {
    println!(" * * * * {:?} {:?} {:?} {:?}", mnemonic, operand1, operand2, operand3);
    let instr = Instruction {
        mnemonic: mnemonic,
        operand1: Some(operand1),
        operand2: Some(operand2),
        operand3: Some(operand3),
        operand4: None,
        flags: Default::default(),
    };
    encode32_helper(&instr, expected);
}

fn encode32_helper3_flags(mnemonic: Mnemonic, operand1: Operand, operand2: Operand, operand3: Operand, flags: InstructionFlags, expected: &Vec<u8>) {
    println!(" * * * * {:?} {:?} {:?} {:?} {:?}", mnemonic, operand1, operand2, operand3, flags);
    let instr = Instruction {
        mnemonic: mnemonic,
        operand1: Some(operand1),
        operand2: Some(operand2),
        operand3: Some(operand3),
        operand4: None,
        flags: flags,
    };
    encode32_helper(&instr, expected);
}

fn encode32_helper4(mnemonic: Mnemonic, operand1: Operand, operand2: Operand, operand3: Operand, operand4: Operand, expected: &Vec<u8>) {
    println!(" * * * * {:?} {:?} {:?} {:?} {:?}", mnemonic, operand1, operand2, operand3, operand4);
    let instr = Instruction {
        mnemonic: mnemonic,
        operand1: Some(operand1),
        operand2: Some(operand2),
        operand3: Some(operand3),
        operand4: Some(operand4),
        flags: Default::default()
    };
    encode32_helper(&instr, expected);
}

fn encode32_assert_ambiguous(mnemonic: Mnemonic, operand1: Option<Operand>, operand2: Option<Operand>, operand3: Option<Operand>, operand4: Option<Operand>) {
    let instr = Instruction {
        mnemonic: mnemonic,
        operand1: operand1,
        operand2: operand2,
        operand3: operand3,
        operand4: operand4,
        flags: Default::default()
    };
    let mut buffer = Cursor::new(Vec::new());
    assert_eq!(instr.encode(&mut buffer, Mode::Protected, ProcessorLevel::Corei7).err(), Some(InstructionEncodingError::AmbiguousSize));
}

fn encode64_helper(instr: &Instruction, expected: &Vec<u8>) {
    let mut buffer = Cursor::new(Vec::new());
    instr.encode(&mut buffer, Mode::Long, ProcessorLevel::Corei7).expect("Encoding failed");
    assert_eq!(buffer.get_ref(), expected);
}

fn encode64_helper2(mnemonic: Mnemonic, operand1: Operand, operand2: Operand, expected: &Vec<u8>) {
    println!(" * * * * {:?} {:?} {:?}", mnemonic, operand1, operand2);
    let instr = Instruction {
        mnemonic: mnemonic,
        operand1: Some(operand1),
        operand2: Some(operand2),
        operand3: None,
        operand4: None,
        flags: Default::default()
    };
    encode64_helper(&instr, expected);
}

