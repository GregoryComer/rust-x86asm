use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp14ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 76, 254], OperandSize::Dword)
}

#[test]
fn vrcp14ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 235430816, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 76, 28, 85, 160, 99, 8, 14], OperandSize::Dword)
}

#[test]
fn vrcp14ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 159, 76, 28, 240], OperandSize::Dword)
}

#[test]
fn vrcp14ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 18, 125, 141, 76, 202], OperandSize::Qword)
}

#[test]
fn vrcp14ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM27)), operand2: Some(IndirectDisplaced(RBX, 498198453, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 125, 143, 76, 155, 181, 231, 177, 29], OperandSize::Qword)
}

#[test]
fn vrcp14ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM9)), operand2: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 125, 156, 76, 8], OperandSize::Qword)
}

#[test]
fn vrcp14ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 76, 237], OperandSize::Dword)
}

#[test]
fn vrcp14ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 160571195, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 76, 20, 133, 59, 31, 146, 9], OperandSize::Dword)
}

#[test]
fn vrcp14ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM2)), operand2: Some(IndirectDisplaced(EDX, 1079295162, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 187, 76, 146, 186, 188, 84, 64], OperandSize::Dword)
}

#[test]
fn vrcp14ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 125, 174, 76, 246], OperandSize::Qword)
}

#[test]
fn vrcp14ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM27)), operand2: Some(IndirectDisplaced(RCX, 982934617, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 125, 171, 76, 153, 89, 100, 150, 58], OperandSize::Qword)
}

#[test]
fn vrcp14ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 187, 76, 44, 83], OperandSize::Qword)
}

#[test]
fn vrcp14ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 76, 200], OperandSize::Dword)
}

#[test]
fn vrcp14ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM7)), operand2: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 76, 63], OperandSize::Dword)
}

#[test]
fn vrcp14ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 880758797, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 220, 76, 60, 253, 13, 80, 127, 52], OperandSize::Dword)
}

#[test]
fn vrcp14ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 125, 204, 76, 241], OperandSize::Qword)
}

#[test]
fn vrcp14ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM26)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 2075930435, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 125, 205, 76, 148, 203, 67, 47, 188, 123], OperandSize::Qword)
}

#[test]
fn vrcp14ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 1202681607, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 217, 76, 20, 133, 7, 119, 175, 71], OperandSize::Qword)
}

