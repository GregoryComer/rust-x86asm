use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xsave_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVE, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 150, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 163, 150, 0], OperandSize::Word)
}

#[test]
fn xsave_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVE, operand1: Some(Indirect(EAX, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 32], OperandSize::Dword)
}

#[test]
fn xsave_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVE, operand1: Some(IndirectScaledDisplaced(RDI, Two, 1379330813, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 36, 125, 253, 234, 54, 82], OperandSize::Qword)
}

