use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 22, 193], OperandSize::Dword)
}

#[test]
fn vpermps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 22, 10], OperandSize::Dword)
}

#[test]
fn vpermps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 22, 248], OperandSize::Qword)
}

#[test]
fn vpermps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 22, 28, 179], OperandSize::Qword)
}

#[test]
fn vpermps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 175, 22, 246], OperandSize::Dword)
}

#[test]
fn vpermps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EDX, 183188337, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 170, 22, 186, 113, 59, 235, 10], OperandSize::Dword)
}

#[test]
fn vpermps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDX, 317338232, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 188, 22, 170, 120, 50, 234, 18], OperandSize::Dword)
}

#[test]
fn vpermps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 53, 171, 22, 195], OperandSize::Qword)
}

#[test]
fn vpermps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectDisplaced(RBX, 893027896, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 53, 170, 22, 139, 56, 134, 58, 53], OperandSize::Qword)
}

#[test]
fn vpermps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 5, 178, 22, 28, 159], OperandSize::Qword)
}

#[test]
fn vpermps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 207, 22, 234], OperandSize::Dword)
}

#[test]
fn vpermps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 203, 22, 44, 115], OperandSize::Dword)
}

#[test]
fn vpermps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 109, 222, 22, 3], OperandSize::Dword)
}

#[test]
fn vpermps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 66, 101, 199, 22, 235], OperandSize::Qword)
}

#[test]
fn vpermps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 1162436940, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 45, 195, 22, 172, 70, 76, 97, 73, 69], OperandSize::Qword)
}

#[test]
fn vpermps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM14)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 13, 221, 22, 14], OperandSize::Qword)
}

