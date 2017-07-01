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

fn encode16_helper_ds(mnemonic: Mnemonic, destination: Operand, source: Operand, expected: &Vec<u8>) {
    println!(" * * * * {:?} {:?} {:?}", mnemonic, destination, source);
    let instr = Instruction {
        mnemonic: mnemonic,
        source: Some(source),
        source2: None,
        source3: None,
        destination: Some(destination),
        flags: Default::default()
    };
    encode16_helper(&instr, expected);
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
        flags: Default::default()
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
        flags: Default::default()
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
        flags: Default::default()
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
        flags: Default::default()
    };
    encode32_helper(&instr, expected);
}

fn encode32_helper_ss_flags(mnemonic: Mnemonic, source: Operand, source2: Operand, flags: InstructionFlags, expected: &Vec<u8>) {
    println!(" * * * * {:?} {:?} {:?} {:?}", mnemonic, source, source2, flags);
    let instr = Instruction {
        mnemonic: mnemonic,
        source: Some(source),
        source2: Some(source2),
        source3: None,
        destination: None,
        flags: flags
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
        flags: Default::default()
    };
    encode32_helper(&instr, expected);
}

fn encode32_helper_ds_flags(mnemonic: Mnemonic, destination: Operand, source: Operand, flags: InstructionFlags, expected: &Vec<u8>) {
    println!(" * * * * {:?} {:?} {:?} {:?}", mnemonic, destination, source, flags);
    let instr = Instruction {
        mnemonic: mnemonic,
        source: Some(source),
        source2: None,
        source3: None,
        destination: Some(destination),
        flags: flags
    };
    encode32_helper(&instr, expected);
}

fn encode32_helper_dss(mnemonic: Mnemonic, destination: Operand, source: Operand, source2: Operand, expected: &Vec<u8>) {
    println!(" * * * * {:?} {:?} {:?} {:?}", mnemonic, destination, source, source2);
    let instr = Instruction {
        mnemonic: mnemonic,
        source: Some(source),
        source2: Some(source2),
        source3: None,
        destination: Some(destination),
        flags: Default::default(),
    };
    encode32_helper(&instr, expected);
}

fn encode32_helper_dss_flags(mnemonic: Mnemonic, destination: Operand, source: Operand, source2: Operand, flags: InstructionFlags, expected: &Vec<u8>) {
    println!(" * * * * {:?} {:?} {:?} {:?} {:?}", mnemonic, destination, source, source2, flags);
    let instr = Instruction {
        mnemonic: mnemonic,
        source: Some(source),
        source2: Some(source2),
        source3: None,
        destination: Some(destination),
        flags: flags,
    };
    encode32_helper(&instr, expected);
}

fn encode32_helper_dsss(mnemonic: Mnemonic, destination: Operand, source: Operand, source2: Operand, source3: Operand, expected: &Vec<u8>) {
    println!(" * * * * {:?} {:?} {:?} {:?} {:?}", mnemonic, destination, source, source2, source3);
    let instr = Instruction {
        mnemonic: mnemonic,
        source: Some(source),
        source2: Some(source2),
        source3: Some(source3),
        destination: Some(destination),
        flags: Default::default()
    };
    encode32_helper(&instr, expected);
}

fn encode32_helper_sss(mnemonic: Mnemonic, source: Operand, source2: Operand, source3: Operand, expected: &Vec<u8>) {
    println!(" * * * * {:?} {:?} {:?} {:?}", mnemonic, source, source2, source3);
    let instr = Instruction {
        mnemonic: mnemonic,
        source: Some(source),
        source2: Some(source2),
        source3: Some(source3),
        destination: None,
        flags: Default::default()
    };
    encode32_helper(&instr, expected);
}

fn encode32_assert_ambiguous(mnemonic: Mnemonic, destination: Option<Operand>, source: Option<Operand>, source2: Option<Operand>, source3: Option<Operand>) {
    let instr = Instruction {
        mnemonic: mnemonic,
        source: source,
        source2: source2,
        source3: source3,
        destination: destination,
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

fn encode64_helper_ds(mnemonic: Mnemonic, destination: Operand, source: Operand, expected: &Vec<u8>) {
    println!(" * * * * {:?} {:?} {:?}", mnemonic, destination, source);
    let instr = Instruction {
        mnemonic: mnemonic,
        source: Some(source),
        source2: None,
        source3: None,
        destination: Some(destination),
        flags: Default::default()
    };
    encode64_helper(&instr, expected);
}

