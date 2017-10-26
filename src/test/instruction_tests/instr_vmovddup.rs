use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovddup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 18, 212], OperandSize::Dword)
}

#[test]
fn vmovddup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EAX, ESI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 18, 20, 176], OperandSize::Dword)
}

#[test]
fn vmovddup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 18, 211], OperandSize::Qword)
}

#[test]
fn vmovddup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RCX, 1399603450, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 18, 137, 250, 64, 108, 83], OperandSize::Qword)
}

#[test]
fn vmovddup_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 18, 243], OperandSize::Dword)
}

#[test]
fn vmovddup_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 1099335063, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 18, 28, 157, 151, 133, 134, 65], OperandSize::Dword)
}

#[test]
fn vmovddup_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 18, 252], OperandSize::Qword)
}

#[test]
fn vmovddup_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 1023074917, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 18, 132, 87, 101, 226, 250, 60], OperandSize::Qword)
}

#[test]
fn vmovddup_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 255, 139, 18, 221], OperandSize::Dword)
}

#[test]
fn vmovddup_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EDX, EDI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 255, 138, 18, 44, 250], OperandSize::Dword)
}

#[test]
fn vmovddup_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 193, 255, 137, 18, 195], OperandSize::Qword)
}

#[test]
fn vmovddup_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM31)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 255, 141, 18, 60, 154], OperandSize::Qword)
}

#[test]
fn vmovddup_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 255, 171, 18, 201], OperandSize::Dword)
}

#[test]
fn vmovddup_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 338026213, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 255, 175, 18, 28, 149, 229, 222, 37, 20], OperandSize::Dword)
}

#[test]
fn vmovddup_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 255, 172, 18, 195], OperandSize::Qword)
}

#[test]
fn vmovddup_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 255, 172, 18, 44, 194], OperandSize::Qword)
}

#[test]
fn vmovddup_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 255, 204, 18, 221], OperandSize::Dword)
}

#[test]
fn vmovddup_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 1181672761, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 255, 206, 18, 20, 149, 57, 229, 110, 70], OperandSize::Dword)
}

#[test]
fn vmovddup_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 255, 205, 18, 242], OperandSize::Qword)
}

#[test]
fn vmovddup_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(ZMM5)), operand2: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 255, 205, 18, 40], OperandSize::Qword)
}

