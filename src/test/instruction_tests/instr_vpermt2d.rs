use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermt2d_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 141, 126, 230], OperandSize::Dword)
}

#[test]
fn vpermt2d_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 143, 126, 36, 219], OperandSize::Dword)
}

#[test]
fn vpermt2d_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1294224939, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 157, 126, 4, 69, 43, 78, 36, 77], OperandSize::Dword)
}

#[test]
fn vpermt2d_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 2, 85, 131, 126, 219], OperandSize::Qword)
}

#[test]
fn vpermt2d_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 85, 143, 126, 48], OperandSize::Qword)
}

#[test]
fn vpermt2d_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 13, 148, 126, 44, 210], OperandSize::Qword)
}

#[test]
fn vpermt2d_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 173, 126, 235], OperandSize::Dword)
}

#[test]
fn vpermt2d_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 727592594, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 172, 126, 172, 191, 146, 46, 94, 43], OperandSize::Dword)
}

#[test]
fn vpermt2d_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 1873463200, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 190, 126, 172, 86, 160, 199, 170, 111], OperandSize::Dword)
}

#[test]
fn vpermt2d_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM19)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 101, 161, 126, 242], OperandSize::Qword)
}

#[test]
fn vpermt2d_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RBX, 2091077182, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 93, 175, 126, 155, 62, 78, 163, 124], OperandSize::Qword)
}

#[test]
fn vpermt2d_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 807646860, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 125, 178, 126, 28, 69, 140, 182, 35, 48], OperandSize::Qword)
}

#[test]
fn vpermt2d_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 126, 197], OperandSize::Dword)
}

#[test]
fn vpermt2d_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(EAX, 1666894673, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 203, 126, 160, 81, 203, 90, 99], OperandSize::Dword)
}

#[test]
fn vpermt2d_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EDX, 1160895677, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 221, 126, 146, 189, 220, 49, 69], OperandSize::Dword)
}

#[test]
fn vpermt2d_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM10)), operand3: Some(Direct(ZMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 2, 45, 201, 126, 239], OperandSize::Qword)
}

#[test]
fn vpermt2d_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(RAX, 1459493443, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 101, 204, 126, 176, 67, 26, 254, 86], OperandSize::Qword)
}

#[test]
fn vpermt2d_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(RCX, 1526074147, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 85, 223, 126, 137, 35, 11, 246, 90], OperandSize::Qword)
}

