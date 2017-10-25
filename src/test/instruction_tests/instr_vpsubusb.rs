use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubusb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 216, 203], OperandSize::Dword)
}

#[test]
fn vpsubusb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 216, 12, 215], OperandSize::Dword)
}

#[test]
fn vpsubusb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 216, 242], OperandSize::Qword)
}

#[test]
fn vpsubusb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RSI, 35528342, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 216, 134, 150, 30, 30, 2], OperandSize::Qword)
}

#[test]
fn vpsubusb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 216, 204], OperandSize::Dword)
}

#[test]
fn vpsubusb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Four, 2043720631, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 216, 180, 185, 183, 179, 208, 121], OperandSize::Dword)
}

#[test]
fn vpsubusb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 216, 216], OperandSize::Qword)
}

#[test]
fn vpsubusb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 269739285, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 216, 52, 197, 21, 229, 19, 16], OperandSize::Qword)
}

#[test]
fn vpsubusb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 141, 216, 232], OperandSize::Dword)
}

#[test]
fn vpsubusb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDI, 2117935738, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 101, 137, 216, 159, 122, 34, 61, 126], OperandSize::Dword)
}

#[test]
fn vpsubusb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 13, 130, 216, 231], OperandSize::Qword)
}

#[test]
fn vpsubusb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 598417591, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 93, 139, 216, 164, 223, 183, 32, 171, 35], OperandSize::Qword)
}

#[test]
fn vpsubusb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 85, 169, 216, 211], OperandSize::Dword)
}

#[test]
fn vpsubusb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 913406873, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 172, 216, 44, 117, 153, 123, 113, 54], OperandSize::Dword)
}

#[test]
fn vpsubusb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 5, 173, 216, 212], OperandSize::Qword)
}

#[test]
fn vpsubusb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM9)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 53, 172, 216, 25], OperandSize::Qword)
}

#[test]
fn vpsubusb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 206, 216, 223], OperandSize::Dword)
}

#[test]
fn vpsubusb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 666120693, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 207, 216, 148, 151, 245, 49, 180, 39], OperandSize::Dword)
}

#[test]
fn vpsubusb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM14)), operand3: Some(Direct(ZMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 17, 13, 205, 216, 200], OperandSize::Qword)
}

#[test]
fn vpsubusb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1595973563, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 29, 202, 216, 52, 157, 187, 159, 32, 95], OperandSize::Qword)
}

