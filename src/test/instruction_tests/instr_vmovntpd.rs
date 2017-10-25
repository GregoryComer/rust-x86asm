use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovntpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 1718840591, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 43, 180, 90, 15, 109, 115, 102], OperandSize::Dword)
}

#[test]
fn vmovntpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 1100546651, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 43, 20, 213, 91, 2, 153, 65], OperandSize::Qword)
}

#[test]
fn vmovntpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 599423145, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 43, 60, 221, 169, 120, 186, 35], OperandSize::Dword)
}

#[test]
fn vmovntpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 43, 15], OperandSize::Qword)
}

#[test]
fn vmovntpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectDisplaced(EDX, 1641918158, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 43, 138, 206, 174, 221, 97], OperandSize::Dword)
}

#[test]
fn vmovntpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectDisplaced(RDX, 1915865134, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 253, 8, 43, 130, 46, 200, 49, 114], OperandSize::Qword)
}

#[test]
fn vmovntpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledDisplaced(EAX, Eight, 562790374, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 43, 36, 197, 230, 127, 139, 33], OperandSize::Dword)
}

#[test]
fn vmovntpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 904212704, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 253, 40, 43, 148, 152, 224, 48, 229, 53], OperandSize::Qword)
}

#[test]
fn vmovntpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 72, 43, 44, 70], OperandSize::Dword)
}

#[test]
fn vmovntpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 253, 72, 43, 6], OperandSize::Qword)
}

