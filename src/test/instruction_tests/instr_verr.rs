use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn verr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 227], OperandSize::Word)
}

#[test]
fn verr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 34], OperandSize::Word)
}

#[test]
fn verr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 226], OperandSize::Dword)
}

#[test]
fn verr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(IndirectScaledIndexed(ECX, ECX, Four, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 36, 137], OperandSize::Dword)
}

#[test]
fn verr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 231], OperandSize::Qword)
}

#[test]
fn verr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 92115712, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 36, 205, 0, 147, 125, 5], OperandSize::Qword)
}

