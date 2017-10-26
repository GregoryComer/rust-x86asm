use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtps2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 90, 223], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 398288348, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 90, 44, 157, 220, 101, 189, 23], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 90, 209], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RDX, RDI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 90, 60, 250], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 90, 241], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Two, 1040654248, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 90, 140, 91, 168, 31, 7, 62], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 90, 206], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 90, 2], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 141, 90, 221], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 139, 90, 52, 255], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 124, 143, 90, 234], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 63801894, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 143, 90, 188, 128, 38, 138, 205, 3], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 172, 90, 240], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Four, 1950687799, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 173, 90, 188, 137, 55, 34, 69, 116], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(XMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 129, 124, 172, 90, 211], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM31)), operand2: Some(IndirectDisplaced(RBX, 1550439897, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 124, 173, 90, 187, 217, 213, 105, 92], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 153, 90, 227], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Four, 1895577318, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 203, 90, 180, 129, 230, 54, 252, 112], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(YMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 161, 124, 158, 90, 206], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(ZMM10)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 879233854, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 124, 202, 90, 20, 253, 62, 11, 104, 52], OperandSize::Qword)
}

