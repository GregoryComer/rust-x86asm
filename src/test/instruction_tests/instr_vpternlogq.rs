use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpternlogq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(109)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 221, 138, 37, 205, 109], OperandSize::Dword)
}

#[test]
fn vpternlogq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(ESI, 959747684, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(48)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 213, 139, 37, 182, 100, 150, 52, 57, 48], OperandSize::Dword)
}

#[test]
fn vpternlogq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDX, 712905004, Some(OperandSize::Qword), None)), operand4: Some(Literal8(113)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 205, 156, 37, 130, 44, 17, 126, 42, 113], OperandSize::Dword)
}

#[test]
fn vpternlogq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM19)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 163, 141, 139, 37, 219, 59], OperandSize::Qword)
}

#[test]
fn vpternlogq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 1088783292, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(81)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 115, 205, 134, 37, 172, 201, 188, 131, 229, 64, 81], OperandSize::Qword)
}

#[test]
fn vpternlogq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectDisplaced(RBX, 590368175, Some(OperandSize::Qword), None)), operand4: Some(Literal8(115)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 229, 151, 37, 131, 175, 77, 48, 35, 115], OperandSize::Qword)
}

#[test]
fn vpternlogq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(50)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 245, 172, 37, 225, 50], OperandSize::Dword)
}

#[test]
fn vpternlogq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 398124675, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(28)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 213, 170, 37, 36, 117, 131, 230, 186, 23, 28], OperandSize::Dword)
}

#[test]
fn vpternlogq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: Some(Literal8(24)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 237, 191, 37, 15, 24], OperandSize::Dword)
}

#[test]
fn vpternlogq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM29)), operand4: Some(Literal8(127)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 19, 165, 163, 37, 197, 127], OperandSize::Qword)
}

#[test]
fn vpternlogq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM19)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(71)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 229, 167, 37, 51, 71], OperandSize::Qword)
}

#[test]
fn vpternlogq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1623936984, Some(OperandSize::Qword), None)), operand4: Some(Literal8(2)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 115, 229, 177, 37, 36, 197, 216, 79, 203, 96, 2], OperandSize::Qword)
}

#[test]
fn vpternlogq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM6)), operand4: Some(Literal8(75)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 205, 201, 37, 198, 75], OperandSize::Dword)
}

#[test]
fn vpternlogq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Two, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(56)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 213, 207, 37, 60, 86, 56], OperandSize::Dword)
}

#[test]
fn vpternlogq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(63)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 229, 220, 37, 34, 63], OperandSize::Dword)
}

#[test]
fn vpternlogq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(ZMM13)), operand4: Some(Literal8(0)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 211, 221, 198, 37, 197, 0], OperandSize::Qword)
}

#[test]
fn vpternlogq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM27)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(86)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 165, 199, 37, 42, 86], OperandSize::Qword)
}

#[test]
fn vpternlogq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectDisplaced(RBX, 1159231312, Some(OperandSize::Qword), None)), operand4: Some(Literal8(99)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 173, 209, 37, 187, 80, 119, 24, 69, 99], OperandSize::Qword)
}

