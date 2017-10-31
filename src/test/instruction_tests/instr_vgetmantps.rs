use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetmantps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 139, 38, 254, 111], OperandSize::Dword)
}

#[test]
fn vgetmantps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 138, 38, 56, 73], OperandSize::Dword)
}

#[test]
fn vgetmantps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 2062272318, Some(OperandSize::Dword), None)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 159, 38, 36, 141, 62, 199, 235, 122, 95], OperandSize::Dword)
}

#[test]
fn vgetmantps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM24)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 147, 125, 139, 38, 216, 79], OperandSize::Qword)
}

#[test]
fn vgetmantps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 142, 38, 36, 72, 56], OperandSize::Qword)
}

#[test]
fn vgetmantps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM26)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Dword), None)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 99, 125, 153, 38, 20, 144, 80], OperandSize::Qword)
}

#[test]
fn vgetmantps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 173, 38, 216, 61], OperandSize::Dword)
}

#[test]
fn vgetmantps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(64)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 174, 38, 20, 209, 64], OperandSize::Dword)
}

#[test]
fn vgetmantps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Two, 275304158, Some(OperandSize::Dword), None)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 188, 38, 164, 81, 222, 206, 104, 16, 13], OperandSize::Dword)
}

#[test]
fn vgetmantps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM8)), operand3: Some(Literal8(28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 83, 125, 172, 38, 232, 28], OperandSize::Qword)
}

#[test]
fn vgetmantps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM20)), operand2: Some(IndirectDisplaced(RSI, 1099125525, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 227, 125, 172, 38, 166, 21, 83, 131, 65, 91], OperandSize::Qword)
}

#[test]
fn vgetmantps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM27)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 1716469743, Some(OperandSize::Dword), None)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 99, 125, 191, 38, 156, 118, 239, 63, 79, 102, 3], OperandSize::Qword)
}

#[test]
fn vgetmantps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 158, 38, 236, 117], OperandSize::Dword)
}

#[test]
fn vgetmantps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM5)), operand2: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 204, 38, 42, 45], OperandSize::Dword)
}

#[test]
fn vgetmantps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM6)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 220, 38, 50, 72], OperandSize::Dword)
}

#[test]
fn vgetmantps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM11)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 211, 125, 158, 38, 243, 108], OperandSize::Qword)
}

#[test]
fn vgetmantps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1813114320, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 207, 38, 12, 141, 208, 237, 17, 108, 124], OperandSize::Qword)
}

#[test]
fn vgetmantps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 505271463, Some(OperandSize::Dword), None)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 220, 38, 60, 93, 167, 212, 29, 30, 90], OperandSize::Qword)
}

