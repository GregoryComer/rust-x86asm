use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovlpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 184262659, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 18, 28, 117, 3, 160, 251, 10], OperandSize::Dword)
}

#[test]
fn vmovlpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RCX, 817435055, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 18, 161, 175, 17, 185, 48], OperandSize::Qword)
}

#[test]
fn vmovlpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(ECX, 267609708, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 18, 177, 108, 102, 243, 15], OperandSize::Dword)
}

#[test]
fn vmovlpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 955821163, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 229, 0, 18, 20, 77, 107, 172, 248, 56], OperandSize::Qword)
}

#[test]
fn vmovlpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Four, 1896948757, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 19, 172, 147, 21, 36, 17, 113], OperandSize::Dword)
}

#[test]
fn vmovlpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(IndirectDisplaced(RAX, 387011766, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 19, 184, 182, 84, 17, 23], OperandSize::Qword)
}

#[test]
fn vmovlpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(IndirectScaledIndexed(EAX, EAX, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 19, 28, 128], OperandSize::Dword)
}

#[test]
fn vmovlpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 157584644, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 253, 8, 19, 36, 197, 4, 141, 100, 9], OperandSize::Qword)
}

