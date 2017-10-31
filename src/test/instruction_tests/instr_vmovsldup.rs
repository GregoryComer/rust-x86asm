use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovsldup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 18, 230], OperandSize::Dword)
}

#[test]
fn vmovsldup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 18, 0], OperandSize::Dword)
}

#[test]
fn vmovsldup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 18, 196], OperandSize::Qword)
}

#[test]
fn vmovsldup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 18, 57], OperandSize::Qword)
}

#[test]
fn vmovsldup_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 18, 202], OperandSize::Dword)
}

#[test]
fn vmovsldup_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM1)), operand2: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 18, 8], OperandSize::Dword)
}

#[test]
fn vmovsldup_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 18, 224], OperandSize::Qword)
}

#[test]
fn vmovsldup_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 582139346, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 18, 36, 197, 210, 189, 178, 34], OperandSize::Qword)
}

#[test]
fn vmovsldup_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 138, 18, 233], OperandSize::Dword)
}

#[test]
fn vmovsldup_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 138, 18, 15], OperandSize::Dword)
}

#[test]
fn vmovsldup_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 209, 126, 141, 18, 214], OperandSize::Qword)
}

#[test]
fn vmovsldup_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM17)), operand2: Some(IndirectDisplaced(RBX, 2066977038, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 126, 138, 18, 139, 14, 145, 51, 123], OperandSize::Qword)
}

#[test]
fn vmovsldup_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 171, 18, 248], OperandSize::Dword)
}

#[test]
fn vmovsldup_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(ESI, 30928739, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 174, 18, 158, 99, 239, 215, 1], OperandSize::Dword)
}

#[test]
fn vmovsldup_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 169, 18, 253], OperandSize::Qword)
}

#[test]
fn vmovsldup_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM28)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 1664441248, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 126, 171, 18, 36, 181, 160, 91, 53, 99], OperandSize::Qword)
}

#[test]
fn vmovsldup_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 205, 18, 250], OperandSize::Dword)
}

#[test]
fn vmovsldup_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 201, 18, 12, 120], OperandSize::Dword)
}

#[test]
fn vmovsldup_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 126, 206, 18, 229], OperandSize::Qword)
}

#[test]
fn vmovsldup_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(ZMM9)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 2118739614, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 126, 205, 18, 140, 95, 158, 102, 73, 126], OperandSize::Qword)
}

