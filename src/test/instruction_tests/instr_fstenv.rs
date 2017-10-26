use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fstenv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTENV, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 232, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 217, 177, 232, 0], OperandSize::Word)
}

#[test]
fn fstenv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTENV, operand1: Some(IndirectScaledDisplaced(EAX, Two, 1442505303, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 217, 52, 69, 87, 226, 250, 85], OperandSize::Dword)
}

#[test]
fn fstenv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTENV, operand1: Some(Indirect(RDX, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 217, 50], OperandSize::Qword)
}

