use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vhsubpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 125, 218], OperandSize::Dword)
}

#[test]
fn vhsubpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Eight, 948390516, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 125, 164, 202, 116, 74, 135, 56], OperandSize::Dword)
}

#[test]
fn vhsubpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 125, 239], OperandSize::Qword)
}

#[test]
fn vhsubpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RCX, 571124482, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 125, 145, 2, 171, 10, 34], OperandSize::Qword)
}

#[test]
fn vhsubpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 125, 210], OperandSize::Dword)
}

#[test]
fn vhsubpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1042224399, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 125, 20, 157, 15, 21, 31, 62], OperandSize::Dword)
}

#[test]
fn vhsubpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 125, 219], OperandSize::Qword)
}

#[test]
fn vhsubpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 1843487115, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 125, 156, 88, 139, 97, 225, 109], OperandSize::Qword)
}

