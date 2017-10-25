use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrangeps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 69, 137, 80, 240, 70], OperandSize::Dword)
}

#[test]
fn vrangeps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 103095566, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(38)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 109, 139, 80, 132, 80, 14, 29, 37, 6, 38], OperandSize::Dword)
}

#[test]
fn vrangeps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Eight, 1138292037, Some(OperandSize::Dword), None)), operand4: Some(Literal8(63)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 93, 155, 80, 188, 246, 69, 245, 216, 67, 63], OperandSize::Dword)
}

#[test]
fn vrangeps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM24)), operand4: Some(Literal8(60)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 19, 61, 140, 80, 248, 60], OperandSize::Qword)
}

#[test]
fn vrangeps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM18)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(53)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 227, 109, 129, 80, 41, 53], OperandSize::Qword)
}

#[test]
fn vrangeps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM31)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(5)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 5, 146, 80, 8, 5], OperandSize::Qword)
}

#[test]
fn vrangeps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(110)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 85, 172, 80, 244, 110], OperandSize::Dword)
}

#[test]
fn vrangeps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(102)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 93, 169, 80, 16, 102], OperandSize::Dword)
}

#[test]
fn vrangeps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EBX, 1874577979, Some(OperandSize::Dword), None)), operand4: Some(Literal8(50)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 93, 190, 80, 171, 59, 202, 187, 111, 50], OperandSize::Dword)
}

#[test]
fn vrangeps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM16)), operand3: Some(Direct(YMM15)), operand4: Some(Literal8(75)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 211, 125, 167, 80, 247, 75], OperandSize::Qword)
}

#[test]
fn vrangeps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Four, 732974811, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(87)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 21, 174, 80, 148, 184, 219, 78, 176, 43, 87], OperandSize::Qword)
}

#[test]
fn vrangeps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(109)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 99, 101, 187, 80, 60, 81, 109], OperandSize::Qword)
}

#[test]
fn vrangeps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM7)), operand4: Some(Literal8(94)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 77, 158, 80, 239, 94], OperandSize::Dword)
}

#[test]
fn vrangeps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(110)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 101, 207, 80, 48, 110], OperandSize::Dword)
}

#[test]
fn vrangeps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 222, 80, 3, 17], OperandSize::Dword)
}

#[test]
fn vrangeps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM24)), operand4: Some(Literal8(85)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 3, 29, 154, 80, 224, 85], OperandSize::Qword)
}

#[test]
fn vrangeps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Eight, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(81)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 115, 13, 203, 80, 28, 195, 81], OperandSize::Qword)
}

#[test]
fn vrangeps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Eight, 202798155, Some(OperandSize::Dword), None)), operand4: Some(Literal8(90)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 99, 13, 214, 80, 164, 250, 75, 116, 22, 12, 90], OperandSize::Qword)
}

