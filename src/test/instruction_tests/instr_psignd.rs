use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psignd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 10, 252], OperandSize::Dword)
}

#[test]
fn psignd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(MM7)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 10, 57], OperandSize::Dword)
}

#[test]
fn psignd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 10, 230], OperandSize::Qword)
}

#[test]
fn psignd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(MM0)), operand2: Some(IndirectDisplaced(RBX, 868139744, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 10, 131, 224, 194, 190, 51], OperandSize::Qword)
}

#[test]
fn psignd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 10, 255], OperandSize::Dword)
}

#[test]
fn psignd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 587348621, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 10, 36, 85, 141, 58, 2, 35], OperandSize::Dword)
}

#[test]
fn psignd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 10, 251], OperandSize::Qword)
}

#[test]
fn psignd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 513055636, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 10, 180, 200, 148, 155, 148, 30], OperandSize::Qword)
}

