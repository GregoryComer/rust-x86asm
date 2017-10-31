use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 54, 216], OperandSize::Dword)
}

#[test]
fn vpermd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 244479904, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 54, 36, 117, 160, 119, 146, 14], OperandSize::Dword)
}

#[test]
fn vpermd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 54, 254], OperandSize::Qword)
}

#[test]
fn vpermd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1926712920, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 54, 4, 181, 88, 78, 215, 114], OperandSize::Qword)
}

#[test]
fn vpermd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 77, 173, 54, 233], OperandSize::Dword)
}

#[test]
fn vpermd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 1719181833, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 175, 54, 180, 80, 9, 162, 120, 102], OperandSize::Dword)
}

#[test]
fn vpermd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 385239590, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 85, 191, 54, 36, 93, 38, 74, 246, 22], OperandSize::Dword)
}

#[test]
fn vpermd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(YMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 146, 61, 166, 54, 255], OperandSize::Qword)
}

#[test]
fn vpermd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 29, 173, 54, 36, 250], OperandSize::Qword)
}

#[test]
fn vpermd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectDisplaced(RDX, 1452000531, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 13, 191, 54, 154, 19, 197, 139, 86], OperandSize::Qword)
}

#[test]
fn vpermd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 204, 54, 254], OperandSize::Dword)
}

#[test]
fn vpermd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 54907427, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 117, 204, 54, 44, 157, 35, 210, 69, 3], OperandSize::Dword)
}

#[test]
fn vpermd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 222, 54, 4, 194], OperandSize::Dword)
}

#[test]
fn vpermd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 37, 201, 54, 242], OperandSize::Qword)
}

#[test]
fn vpermd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 1366504231, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 53, 203, 54, 164, 146, 39, 51, 115, 81], OperandSize::Qword)
}

#[test]
fn vpermd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 45, 215, 54, 28, 73], OperandSize::Qword)
}

