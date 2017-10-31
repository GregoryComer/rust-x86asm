use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movapd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 220], OperandSize::Dword)
}

#[test]
fn movapd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 139501072, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 44, 77, 16, 158, 80, 8], OperandSize::Dword)
}

#[test]
fn movapd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 221], OperandSize::Qword)
}

#[test]
fn movapd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 60, 75], OperandSize::Qword)
}

#[test]
fn movapd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 228], OperandSize::Dword)
}

#[test]
fn movapd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 41, 57], OperandSize::Dword)
}

#[test]
fn movapd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 222], OperandSize::Qword)
}

#[test]
fn movapd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(IndirectDisplaced(RAX, 3447741, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 41, 136, 189, 155, 52, 0], OperandSize::Qword)
}

