use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psignw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 9, 248], OperandSize::Dword)
}

#[test]
fn psignw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(MM5)), operand2: Some(IndirectDisplaced(ECX, 1287966367, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 9, 169, 159, 206, 196, 76], OperandSize::Dword)
}

#[test]
fn psignw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 9, 210], OperandSize::Qword)
}

#[test]
fn psignw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexed(RBX, RBX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 9, 36, 91], OperandSize::Qword)
}

#[test]
fn psignw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 9, 223], OperandSize::Dword)
}

#[test]
fn psignw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 9, 52, 143], OperandSize::Dword)
}

#[test]
fn psignw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 9, 204], OperandSize::Qword)
}

#[test]
fn psignw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RAX, RBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 9, 60, 88], OperandSize::Qword)
}

