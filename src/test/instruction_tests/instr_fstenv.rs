use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fstenv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTENV, operand1: Some(IndirectDisplaced(DI, 33, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 217, 117, 33], OperandSize::Word)
}

#[test]
fn fstenv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTENV, operand1: Some(IndirectDisplaced(EBX, 599725176, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 217, 179, 120, 20, 191, 35], OperandSize::Dword)
}

#[test]
fn fstenv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTENV, operand1: Some(Indirect(RBX, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 217, 51], OperandSize::Qword)
}

