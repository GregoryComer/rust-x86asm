use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovntpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 43, 34], OperandSize::Dword)
}

#[test]
fn vmovntpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 1100011543, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 43, 148, 114, 23, 216, 144, 65], OperandSize::Qword)
}

#[test]
fn vmovntpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledDisplaced(EDX, Eight, 1750278239, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 43, 20, 213, 95, 32, 83, 104], OperandSize::Dword)
}

#[test]
fn vmovntpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledDisplaced(RAX, Four, 1055774533, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 43, 52, 133, 69, 215, 237, 62], OperandSize::Qword)
}

#[test]
fn vmovntpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 1474672011, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 43, 132, 177, 139, 181, 229, 87], OperandSize::Dword)
}

#[test]
fn vmovntpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 1470099340, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 43, 12, 245, 140, 239, 159, 87], OperandSize::Qword)
}

#[test]
fn vmovntpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectDisplaced(EAX, 1942840246, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 43, 160, 182, 99, 205, 115], OperandSize::Dword)
}

#[test]
fn vmovntpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 43, 14], OperandSize::Qword)
}

#[test]
fn vmovntpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledDisplaced(ECX, Four, 644457875, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 72, 43, 52, 141, 147, 165, 105, 38], OperandSize::Dword)
}

#[test]
fn vmovntpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectDisplaced(RDI, 1685547518, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 72, 43, 175, 254, 105, 119, 100], OperandSize::Qword)
}

