use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fxsave_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FXSAVE, operand1: Some(Memory(4411, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 6, 59, 17], OperandSize::Word)
}

#[test]
fn fxsave_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FXSAVE, operand1: Some(Indirect(EAX, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 0], OperandSize::Dword)
}

#[test]
fn fxsave_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FXSAVE, operand1: Some(Indirect(RBX, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 3], OperandSize::Qword)
}

