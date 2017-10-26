use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fist_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 4126, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 146, 30, 16], OperandSize::Word)
}

#[test]
fn fist_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(IndirectScaledDisplaced(ESI, Two, 1114402731, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 20, 117, 171, 111, 108, 66], OperandSize::Dword)
}

#[test]
fn fist_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 16], OperandSize::Qword)
}

#[test]
fn fist_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(Indirect(SI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 20], OperandSize::Word)
}

#[test]
fn fist_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(IndirectDisplaced(EDX, 137816482, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 146, 162, 233, 54, 8], OperandSize::Dword)
}

#[test]
fn fist_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(IndirectScaledDisplaced(RAX, Four, 15634376, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 20, 133, 200, 143, 238, 0], OperandSize::Qword)
}

