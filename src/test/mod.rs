//mod addressing16;
//mod addressing32;
//mod addressing64;
//mod decode;
//mod instr32_m_u;
//mod size_inference;
mod instruction_tests;

use std::io::Cursor;
use ::*;
use ::instruction::{Instruction};

fn test_aliased<F>(mnemonics: &[Mnemonic], test: F) 
    where F: Fn(Mnemonic) {
    for mnemonic in mnemonics {
        test(*mnemonic);
    }
}

fn decode_helper(bytes: &Vec<u8>, mode: Mode, expected: &Instruction) {
    let mut buffer = Cursor::new(bytes);
    let mut reader = InstructionReader::new(buffer, mode);
    assert_eq!(reader.read().expect("Decoding failed"), *expected);
}

fn decode32_helper(bytes: &Vec<u8>, expected: &Instruction) {
    decode_helper(bytes, Mode::Protected, expected);
}

fn decode32_helper1(bytes: &Vec<u8>, mnemonic: Mnemonic, operand1: Operand) {
    let instr = Instruction {
        mnemonic: Mnemonic::ADD,
        operand1: Some(operand1),
        .. Default::default()
    };
    decode32_helper(bytes, &instr);
}

fn decode32_helper2(bytes: &Vec<u8>, mnemonic: Mnemonic, operand1: Operand, operand2: Operand) {
    let instr = Instruction {
        mnemonic: Mnemonic::ADD,
        operand1: Some(operand1),
        operand2: Some(operand2),
        .. Default::default()
    };
    decode32_helper(bytes, &instr);
}

fn encode16_helper(instr: &Instruction, expected: &Vec<u8>) {
    let mut buffer = Cursor::new(Vec::new());
    instr.encode(&mut buffer, Mode::Real).expect("Encoding failed");
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
        ..Default::default()
    };
    encode16_helper(&instr, expected);
}

fn encode32_helper(instr: &Instruction, expected: &Vec<u8>) {
    let mut buffer = Cursor::new(Vec::new());
    instr.encode(&mut buffer, Mode::Protected).expect("Encoding failed");
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
        ..Default::default()
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
        ..Default::default()
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
        ..Default::default()
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
        ..Default::default()
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
        ..Default::default()
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
        ..Default::default()
    };
    let mut buffer = Cursor::new(Vec::new());
    assert_eq!(instr.encode(&mut buffer, Mode::Protected).err(), Some(InstructionEncodingError::AmbiguousSize));
}

fn encode64_helper(instr: &Instruction, expected: &Vec<u8>) {
    let mut buffer = Cursor::new(Vec::new());
    instr.encode(&mut buffer, Mode::Long).expect("Encoding failed");
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
        ..Default::default()
    };
    encode64_helper(&instr, expected);
}

fn run_test(instr: &Instruction, expected: &[u8], addr_size: OperandSize) {
    let mut buffer = Cursor::new(Vec::new());
    instr.encode(&mut buffer, Mode::from_size(addr_size).unwrap()).expect("Encoding failed");
    assert_eq!(&buffer.get_ref()[..], expected);
}
