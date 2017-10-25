use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtps2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 90, 249], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 13301502, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 90, 172, 127, 254, 246, 202, 0], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 90, 252], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Eight, 1663164956, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 90, 188, 222, 28, 226, 33, 99], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 90, 202], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectDisplaced(EDI, 2061100456, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 90, 135, 168, 229, 217, 122], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 90, 203], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 90, 62], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 141, 90, 252], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 612439479, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 140, 90, 36, 149, 183, 21, 129, 36], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 124, 137, 90, 204], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM10)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 1752630203, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 124, 142, 90, 148, 75, 187, 3, 119, 104], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 173, 90, 206], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 1240002159, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 174, 90, 172, 119, 111, 238, 232, 73], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 49, 124, 172, 90, 219], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM25)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 2074786596, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 124, 172, 90, 12, 133, 36, 187, 170, 123], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 156, 90, 232], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 2067968875, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 202, 90, 28, 205, 107, 179, 66, 123], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 124, 153, 90, 239], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(ZMM10)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 849021124, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 124, 203, 90, 20, 69, 196, 8, 155, 50], OperandSize::Qword)
}

