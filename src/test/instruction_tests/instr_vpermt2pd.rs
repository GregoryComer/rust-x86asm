use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermt2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 140, 127, 226], OperandSize::Dword)
}

#[test]
fn vpermt2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 141, 127, 60, 152], OperandSize::Dword)
}

#[test]
fn vpermt2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 213, 159, 127, 23], OperandSize::Dword)
}

#[test]
fn vpermt2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 213, 142, 127, 224], OperandSize::Qword)
}

#[test]
fn vpermt2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 752062492, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 213, 134, 127, 12, 213, 28, 144, 211, 44], OperandSize::Qword)
}

#[test]
fn vpermt2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM24)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 189, 151, 127, 57], OperandSize::Qword)
}

#[test]
fn vpermt2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 245, 173, 127, 202], OperandSize::Dword)
}

#[test]
fn vpermt2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 173, 127, 44, 217], OperandSize::Dword)
}

#[test]
fn vpermt2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 245, 189, 127, 60, 130], OperandSize::Dword)
}

#[test]
fn vpermt2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 50, 213, 166, 127, 198], OperandSize::Qword)
}

#[test]
fn vpermt2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 1613355702, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 189, 172, 127, 156, 66, 182, 218, 41, 96], OperandSize::Qword)
}

#[test]
fn vpermt2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 1741144067, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 165, 186, 127, 140, 134, 3, 192, 199, 103], OperandSize::Qword)
}

#[test]
fn vpermt2pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 202, 127, 207], OperandSize::Dword)
}

#[test]
fn vpermt2pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 237, 202, 127, 36, 194], OperandSize::Dword)
}

#[test]
fn vpermt2pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EBX, 1806003646, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 221, 127, 187, 190, 109, 165, 107], OperandSize::Dword)
}

#[test]
fn vpermt2pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM30)), operand3: Some(Direct(ZMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 141, 198, 127, 220], OperandSize::Qword)
}

#[test]
fn vpermt2pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM8)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 189, 205, 127, 62], OperandSize::Qword)
}

#[test]
fn vpermt2pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM23)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 215, 127, 34], OperandSize::Qword)
}

