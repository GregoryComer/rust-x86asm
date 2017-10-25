use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpconflictd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 196, 251], OperandSize::Dword)
}

#[test]
fn vpconflictd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 221336159, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 196, 12, 125, 95, 82, 49, 13], OperandSize::Dword)
}

#[test]
fn vpconflictd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(EDX, 483967609, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 155, 196, 170, 121, 194, 216, 28], OperandSize::Dword)
}

#[test]
fn vpconflictd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 146, 125, 139, 196, 234], OperandSize::Qword)
}

#[test]
fn vpconflictd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1645435764, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 196, 28, 69, 116, 91, 19, 98], OperandSize::Qword)
}

#[test]
fn vpconflictd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM8)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1459199312, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 125, 154, 196, 4, 205, 80, 157, 249, 86], OperandSize::Qword)
}

#[test]
fn vpconflictd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 196, 246], OperandSize::Dword)
}

#[test]
fn vpconflictd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(EDX, 1430026177, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 196, 154, 193, 119, 60, 85], OperandSize::Dword)
}

#[test]
fn vpconflictd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(EDI, 1424039481, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 190, 196, 183, 57, 30, 225, 84], OperandSize::Dword)
}

#[test]
fn vpconflictd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 130, 125, 175, 196, 208], OperandSize::Qword)
}

#[test]
fn vpconflictd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM9)), operand2: Some(IndirectDisplaced(RCX, 1433511496, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 125, 170, 196, 137, 72, 166, 113, 85], OperandSize::Qword)
}

#[test]
fn vpconflictd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM27)), operand2: Some(IndirectDisplaced(RBX, 1424033385, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 125, 189, 196, 155, 105, 6, 225, 84], OperandSize::Qword)
}

#[test]
fn vpconflictd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 196, 207], OperandSize::Dword)
}

#[test]
fn vpconflictd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 476426032, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 196, 28, 85, 48, 175, 101, 28], OperandSize::Dword)
}

#[test]
fn vpconflictd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1456105573, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 221, 196, 4, 181, 101, 104, 202, 86], OperandSize::Dword)
}

#[test]
fn vpconflictd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 194, 125, 205, 196, 220], OperandSize::Qword)
}

#[test]
fn vpconflictd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM31)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 1061861025, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 125, 204, 196, 188, 134, 161, 182, 74, 63], OperandSize::Qword)
}

#[test]
fn vpconflictd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM13)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 125, 223, 196, 43], OperandSize::Qword)
}

