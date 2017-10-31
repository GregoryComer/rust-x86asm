use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fstenv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTENV, operand1: Some(Indirect(BX, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 217, 55], OperandSize::Word)
}

#[test]
fn fstenv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTENV, operand1: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 1563711734, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 217, 180, 75, 246, 88, 52, 93], OperandSize::Dword)
}

#[test]
fn fstenv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTENV, operand1: Some(Indirect(RAX, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 217, 48], OperandSize::Qword)
}

