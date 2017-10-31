use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrangeps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(24)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 109, 140, 80, 224, 24], OperandSize::Dword)
}

#[test]
fn vrangeps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(ECX, 792310084, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(35)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 141, 80, 169, 68, 177, 57, 47, 35], OperandSize::Dword)
}

#[test]
fn vrangeps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Eight, 718065561, Some(OperandSize::Dword), None)), operand4: Some(Literal8(57)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 109, 153, 80, 156, 223, 153, 207, 204, 42, 57], OperandSize::Dword)
}

#[test]
fn vrangeps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM15)), operand4: Some(Literal8(112)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 67, 45, 135, 80, 199, 112], OperandSize::Qword)
}

#[test]
fn vrangeps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(88)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 99, 69, 134, 80, 60, 87, 88], OperandSize::Qword)
}

#[test]
fn vrangeps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RSI, 12891617, Some(OperandSize::Dword), None)), operand4: Some(Literal8(57)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 227, 77, 157, 80, 158, 225, 181, 196, 0, 57], OperandSize::Qword)
}

#[test]
fn vrangeps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(127)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 77, 171, 80, 206, 127], OperandSize::Dword)
}

#[test]
fn vrangeps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Eight, 97094225, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(127)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 69, 169, 80, 132, 249, 81, 138, 201, 5, 127], OperandSize::Dword)
}

#[test]
fn vrangeps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(123)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 69, 188, 80, 20, 121, 123], OperandSize::Dword)
}

#[test]
fn vrangeps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM25)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 131, 117, 171, 80, 225, 17], OperandSize::Qword)
}

#[test]
fn vrangeps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectDisplaced(RSI, 2076588228, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(53)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 227, 21, 162, 80, 150, 196, 56, 198, 123, 53], OperandSize::Qword)
}

#[test]
fn vrangeps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(95)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 227, 117, 189, 80, 28, 194, 95], OperandSize::Qword)
}

#[test]
fn vrangeps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM7)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 69, 157, 80, 199, 82], OperandSize::Dword)
}

#[test]
fn vrangeps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Eight, 705132909, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(60)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 117, 202, 80, 180, 223, 109, 121, 7, 42, 60], OperandSize::Dword)
}

#[test]
fn vrangeps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: Some(Literal8(7)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 77, 221, 80, 22, 7], OperandSize::Dword)
}

#[test]
fn vrangeps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM1)), operand4: Some(Literal8(25)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 99, 85, 151, 80, 201, 25], OperandSize::Qword)
}

#[test]
fn vrangeps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM21)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(49)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 227, 85, 195, 80, 62, 49], OperandSize::Qword)
}

#[test]
fn vrangeps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 2005569880, Some(OperandSize::Dword), None)), operand4: Some(Literal8(66)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 223, 80, 164, 143, 88, 145, 138, 119, 66], OperandSize::Qword)
}

