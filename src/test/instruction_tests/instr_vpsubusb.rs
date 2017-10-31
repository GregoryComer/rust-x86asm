use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubusb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 216, 195], OperandSize::Dword)
}

#[test]
fn vpsubusb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDI, 1963054689, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 216, 151, 97, 214, 1, 117], OperandSize::Dword)
}

#[test]
fn vpsubusb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 216, 232], OperandSize::Qword)
}

#[test]
fn vpsubusb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RAX, 1219918577, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 216, 144, 241, 122, 182, 72], OperandSize::Qword)
}

#[test]
fn vpsubusb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 216, 221], OperandSize::Dword)
}

#[test]
fn vpsubusb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EBX, 606977414, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 216, 171, 134, 189, 45, 36], OperandSize::Dword)
}

#[test]
fn vpsubusb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 216, 208], OperandSize::Qword)
}

#[test]
fn vpsubusb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 216, 32], OperandSize::Qword)
}

#[test]
fn vpsubusb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 85, 137, 216, 235], OperandSize::Dword)
}

#[test]
fn vpsubusb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 140, 216, 15], OperandSize::Dword)
}

#[test]
fn vpsubusb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 33, 69, 134, 216, 201], OperandSize::Qword)
}

#[test]
fn vpsubusb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1605571964, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 141, 216, 28, 77, 124, 21, 179, 95], OperandSize::Qword)
}

#[test]
fn vpsubusb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 172, 216, 210], OperandSize::Dword)
}

#[test]
fn vpsubusb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EAX, 436074274, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 85, 174, 216, 128, 34, 247, 253, 25], OperandSize::Dword)
}

#[test]
fn vpsubusb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 1, 5, 164, 216, 251], OperandSize::Qword)
}

#[test]
fn vpsubusb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectDisplaced(RDX, 1702975126, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 21, 174, 216, 170, 150, 86, 129, 101], OperandSize::Qword)
}

#[test]
fn vpsubusb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 202, 216, 198], OperandSize::Dword)
}

#[test]
fn vpsubusb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Two, 2012967888, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 207, 216, 172, 66, 208, 115, 251, 119], OperandSize::Dword)
}

#[test]
fn vpsubusb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 49, 29, 204, 216, 227], OperandSize::Qword)
}

#[test]
fn vpsubusb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 591245576, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 53, 193, 216, 28, 117, 8, 177, 61, 35], OperandSize::Qword)
}

