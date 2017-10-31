use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp14ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 76, 240], OperandSize::Dword)
}

#[test]
fn vrcp14ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(EAX, ECX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 76, 60, 72], OperandSize::Dword)
}

#[test]
fn vrcp14ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 156, 76, 8], OperandSize::Dword)
}

#[test]
fn vrcp14ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 76, 244], OperandSize::Qword)
}

#[test]
fn vrcp14ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM30)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 125, 137, 76, 50], OperandSize::Qword)
}

#[test]
fn vrcp14ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM17)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 125, 156, 76, 9], OperandSize::Qword)
}

#[test]
fn vrcp14ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 76, 216], OperandSize::Dword)
}

#[test]
fn vrcp14ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 76, 22], OperandSize::Dword)
}

#[test]
fn vrcp14ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 188, 76, 63], OperandSize::Dword)
}

#[test]
fn vrcp14ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 194, 125, 175, 76, 204], OperandSize::Qword)
}

#[test]
fn vrcp14ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM21)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1558890806, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 125, 169, 76, 44, 85, 54, 201, 234, 92], OperandSize::Qword)
}

#[test]
fn vrcp14ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM26)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Four, 537361466, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 125, 185, 76, 148, 154, 58, 124, 7, 32], OperandSize::Qword)
}

#[test]
fn vrcp14ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 76, 216], OperandSize::Dword)
}

#[test]
fn vrcp14ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1244617460, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 76, 52, 69, 244, 90, 47, 74], OperandSize::Dword)
}

#[test]
fn vrcp14ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexed(EDX, EDX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 218, 76, 28, 146], OperandSize::Dword)
}

#[test]
fn vrcp14ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 76, 252], OperandSize::Qword)
}

#[test]
fn vrcp14ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM8)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 676686082, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 125, 203, 76, 4, 141, 2, 105, 85, 40], OperandSize::Qword)
}

#[test]
fn vrcp14ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM7)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 223, 76, 62], OperandSize::Qword)
}

