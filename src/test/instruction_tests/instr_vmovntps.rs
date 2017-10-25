use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovntps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 186306376, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 43, 180, 206, 72, 207, 26, 11], OperandSize::Dword)
}

#[test]
fn vmovntps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledDisplaced(RDI, Two, 1966667729, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 43, 36, 125, 209, 247, 56, 117], OperandSize::Qword)
}

#[test]
fn vmovntps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 940434588, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 43, 36, 221, 156, 228, 13, 56], OperandSize::Dword)
}

#[test]
fn vmovntps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 43, 52, 75], OperandSize::Qword)
}

#[test]
fn vmovntps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectDisplaced(EAX, 352341499, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 43, 136, 251, 77, 0, 21], OperandSize::Dword)
}

#[test]
fn vmovntps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledIndexed(RDX, RSI, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 124, 8, 43, 60, 114], OperandSize::Qword)
}

#[test]
fn vmovntps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 43, 57], OperandSize::Dword)
}

#[test]
fn vmovntps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 417084024, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 124, 43, 20, 205, 120, 50, 220, 24], OperandSize::Qword)
}

#[test]
fn vmovntps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Four, 1047401960, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 124, 72, 43, 140, 158, 232, 21, 110, 62], OperandSize::Dword)
}

#[test]
fn vmovntps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledIndexed(RSI, RDX, Eight, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 124, 72, 43, 12, 214], OperandSize::Qword)
}

