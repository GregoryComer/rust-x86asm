use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fxsave_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FXSAVE, operand1: Some(Indirect(DI, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 5], OperandSize::Word)
}

#[test]
fn fxsave_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FXSAVE, operand1: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 1641026847, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 132, 79, 31, 21, 208, 97], OperandSize::Dword)
}

#[test]
fn fxsave_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FXSAVE, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Eight, 46513198, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 132, 218, 46, 188, 197, 2], OperandSize::Qword)
}

