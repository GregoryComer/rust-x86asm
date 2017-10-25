use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fnstenv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTENV, operand1: Some(IndirectDisplaced(BX, 50, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 119, 50], OperandSize::Word)
}

#[test]
fn fnstenv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTENV, operand1: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 55856024, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 180, 75, 152, 75, 84, 3], OperandSize::Dword)
}

#[test]
fn fnstenv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTENV, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 748560160, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 180, 153, 32, 31, 158, 44], OperandSize::Qword)
}

