use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp14pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 143, 76, 233], OperandSize::Dword)
}

#[test]
fn vrcp14pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 142, 76, 60, 70], OperandSize::Dword)
}

#[test]
fn vrcp14pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(ECX, 1510930369, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 158, 76, 161, 193, 247, 14, 90], OperandSize::Dword)
}

#[test]
fn vrcp14pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 253, 141, 76, 217], OperandSize::Qword)
}

#[test]
fn vrcp14pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM21)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 632383272, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 253, 140, 76, 44, 133, 40, 103, 177, 37], OperandSize::Qword)
}

#[test]
fn vrcp14pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1369010621, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 159, 76, 52, 93, 189, 113, 153, 81], OperandSize::Qword)
}

#[test]
fn vrcp14pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 173, 76, 233], OperandSize::Dword)
}

#[test]
fn vrcp14pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 76, 1], OperandSize::Dword)
}

#[test]
fn vrcp14pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1443156105, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 187, 76, 4, 133, 137, 208, 4, 86], OperandSize::Dword)
}

#[test]
fn vrcp14pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 173, 76, 211], OperandSize::Qword)
}

#[test]
fn vrcp14pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM10)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 253, 174, 76, 20, 137], OperandSize::Qword)
}

#[test]
fn vrcp14pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(RSI, 1078769582, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 191, 76, 182, 174, 183, 76, 64], OperandSize::Qword)
}

#[test]
fn vrcp14pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 76, 221], OperandSize::Dword)
}

#[test]
fn vrcp14pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 1897076927, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 206, 76, 36, 141, 191, 24, 19, 113], OperandSize::Dword)
}

#[test]
fn vrcp14pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectDisplaced(EAX, 171841350, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 221, 76, 152, 70, 23, 62, 10], OperandSize::Dword)
}

#[test]
fn vrcp14pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 203, 76, 252], OperandSize::Qword)
}

#[test]
fn vrcp14pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM27)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 449610584, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 253, 201, 76, 28, 77, 88, 131, 204, 26], OperandSize::Qword)
}

#[test]
fn vrcp14pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 2087625753, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 223, 76, 44, 133, 25, 164, 110, 124], OperandSize::Qword)
}

