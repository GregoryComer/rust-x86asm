use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetmantpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 138, 38, 245, 32], OperandSize::Dword)
}

#[test]
fn vgetmantpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 549154300, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 143, 38, 164, 178, 252, 109, 187, 32, 80], OperandSize::Dword)
}

#[test]
fn vgetmantpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 1768832192, Some(OperandSize::Qword), None)), operand3: Some(Literal8(28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 159, 38, 172, 131, 192, 60, 110, 105, 28], OperandSize::Dword)
}

#[test]
fn vgetmantpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM8)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 195, 253, 138, 38, 216, 62], OperandSize::Qword)
}

#[test]
fn vgetmantpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM29)), operand2: Some(IndirectDisplaced(RDI, 1772154500, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 99, 253, 138, 38, 175, 132, 238, 160, 105, 107], OperandSize::Qword)
}

#[test]
fn vgetmantpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM24)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 1860919044, Some(OperandSize::Qword), None)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 99, 253, 159, 38, 132, 153, 4, 95, 235, 110, 93], OperandSize::Qword)
}

#[test]
fn vgetmantpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 172, 38, 203, 70], OperandSize::Dword)
}

#[test]
fn vgetmantpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 174, 38, 2, 117], OperandSize::Dword)
}

#[test]
fn vgetmantpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 1249242422, Some(OperandSize::Qword), None)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 190, 38, 44, 221, 54, 237, 117, 74, 72], OperandSize::Dword)
}

#[test]
fn vgetmantpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 115, 253, 169, 38, 223, 2], OperandSize::Qword)
}

#[test]
fn vgetmantpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 780668041, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 170, 38, 60, 117, 137, 12, 136, 46, 19], OperandSize::Qword)
}

#[test]
fn vgetmantpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 187, 38, 20, 246, 73], OperandSize::Qword)
}

#[test]
fn vgetmantpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 154, 38, 219, 49], OperandSize::Dword)
}

#[test]
fn vgetmantpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM5)), operand2: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 206, 38, 40, 24], OperandSize::Dword)
}

#[test]
fn vgetmantpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 455959609, Some(OperandSize::Qword), None)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 223, 38, 28, 181, 57, 100, 45, 27, 36], OperandSize::Dword)
}

#[test]
fn vgetmantpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM25)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 147, 253, 153, 38, 225, 125], OperandSize::Qword)
}

#[test]
fn vgetmantpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM14)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Four, 1465720183, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 115, 253, 205, 38, 180, 178, 119, 29, 93, 87, 72], OperandSize::Qword)
}

#[test]
fn vgetmantpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectDisplaced(RAX, 883925860, Some(OperandSize::Qword), None)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 219, 38, 144, 100, 163, 175, 52, 60], OperandSize::Qword)
}

