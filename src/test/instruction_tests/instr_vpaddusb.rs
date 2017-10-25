use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddusb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 220, 207], OperandSize::Dword)
}

#[test]
fn vpaddusb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 184244194, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 220, 52, 157, 226, 87, 251, 10], OperandSize::Dword)
}

#[test]
fn vpaddusb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 220, 221], OperandSize::Qword)
}

#[test]
fn vpaddusb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RDI, 1917069572, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 220, 159, 4, 41, 68, 114], OperandSize::Qword)
}

#[test]
fn vpaddusb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 220, 211], OperandSize::Dword)
}

#[test]
fn vpaddusb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EDI, 221825911, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 220, 191, 119, 203, 56, 13], OperandSize::Dword)
}

#[test]
fn vpaddusb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 220, 217], OperandSize::Qword)
}

#[test]
fn vpaddusb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 220, 20, 155], OperandSize::Qword)
}

#[test]
fn vpaddusb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 140, 220, 230], OperandSize::Dword)
}

#[test]
fn vpaddusb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 69, 142, 220, 36, 214], OperandSize::Dword)
}

#[test]
fn vpaddusb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 65, 37, 138, 220, 221], OperandSize::Qword)
}

#[test]
fn vpaddusb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM27)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 37, 134, 220, 14], OperandSize::Qword)
}

#[test]
fn vpaddusb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 101, 170, 220, 207], OperandSize::Dword)
}

#[test]
fn vpaddusb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EBX, 1961653709, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 69, 174, 220, 139, 205, 117, 236, 116], OperandSize::Dword)
}

#[test]
fn vpaddusb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 117, 175, 220, 208], OperandSize::Qword)
}

#[test]
fn vpaddusb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM10)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 45, 172, 220, 22], OperandSize::Qword)
}

#[test]
fn vpaddusb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 205, 220, 246], OperandSize::Dword)
}

#[test]
fn vpaddusb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(EDI, 25332355, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 205, 220, 135, 131, 138, 130, 1], OperandSize::Dword)
}

#[test]
fn vpaddusb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 125, 207, 220, 201], OperandSize::Qword)
}

#[test]
fn vpaddusb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 5, 193, 220, 20, 209], OperandSize::Qword)
}

