use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fstenv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTENV, operand1: Some(IndirectDisplaced(BP, 169, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 217, 182, 169, 0], OperandSize::Word)
}

#[test]
fn fstenv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTENV, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Two, 2044014497, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 217, 180, 88, 161, 47, 213, 121], OperandSize::Dword)
}

#[test]
fn fstenv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTENV, operand1: Some(IndirectScaledDisplaced(RSI, Four, 1468228342, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 217, 52, 181, 246, 98, 131, 87], OperandSize::Qword)
}

