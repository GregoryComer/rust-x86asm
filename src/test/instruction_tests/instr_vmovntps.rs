use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovntps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledDisplaced(EBX, Two, 1229033867, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 43, 44, 93, 139, 145, 65, 73], OperandSize::Dword)
}

#[test]
fn vmovntps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledDisplaced(RDX, Two, 295541331, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 43, 60, 85, 83, 154, 157, 17], OperandSize::Qword)
}

#[test]
fn vmovntps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledDisplaced(EDX, Four, 1899903407, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 43, 60, 149, 175, 57, 62, 113], OperandSize::Dword)
}

#[test]
fn vmovntps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 828469525, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 43, 52, 221, 21, 113, 97, 49], OperandSize::Qword)
}

#[test]
fn vmovntps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 888900480, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 43, 188, 243, 128, 139, 251, 52], OperandSize::Dword)
}

#[test]
fn vmovntps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 1870114013, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 124, 8, 43, 148, 114, 221, 172, 119, 111], OperandSize::Qword)
}

#[test]
fn vmovntps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Two, 116112287, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 43, 148, 88, 159, 187, 235, 6], OperandSize::Dword)
}

#[test]
fn vmovntps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledIndexed(RBX, RBX, Two, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 124, 40, 43, 36, 91], OperandSize::Qword)
}

#[test]
fn vmovntps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledDisplaced(ESI, Four, 130374404, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 124, 72, 43, 12, 181, 4, 91, 197, 7], OperandSize::Dword)
}

#[test]
fn vmovntps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledDisplaced(RAX, Two, 363437589, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 124, 72, 43, 60, 69, 21, 158, 169, 21], OperandSize::Qword)
}

