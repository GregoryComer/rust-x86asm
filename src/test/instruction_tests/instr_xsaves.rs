use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xsaves_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVES, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 166, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 171, 166, 0], OperandSize::Word)
}

#[test]
fn xsaves_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVES, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Two, 2104293924, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 172, 70, 36, 250, 108, 125], OperandSize::Dword)
}

#[test]
fn xsaves_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVES, operand1: Some(IndirectScaledIndexed(RBX, RAX, Four, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 44, 131], OperandSize::Qword)
}

