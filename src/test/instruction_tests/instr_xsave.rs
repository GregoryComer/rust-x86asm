use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xsave_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVE, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 35], OperandSize::Word)
}

#[test]
fn xsave_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVE, operand1: Some(IndirectScaledDisplaced(ECX, Eight, 551181552, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 36, 205, 240, 92, 218, 32], OperandSize::Dword)
}

#[test]
fn xsave_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVE, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 200379235, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 164, 190, 99, 139, 241, 11], OperandSize::Qword)
}

