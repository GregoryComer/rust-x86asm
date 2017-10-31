use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermi2d_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 140, 118, 210], OperandSize::Dword)
}

#[test]
fn vpermi2d_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 137, 118, 59], OperandSize::Dword)
}

#[test]
fn vpermi2d_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDI, 1865494849, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 77, 154, 118, 183, 65, 49, 49, 111], OperandSize::Dword)
}

#[test]
fn vpermi2d_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 2, 13, 134, 118, 202], OperandSize::Qword)
}

#[test]
fn vpermi2d_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 141, 118, 25], OperandSize::Qword)
}

#[test]
fn vpermi2d_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 69, 153, 118, 7], OperandSize::Qword)
}

#[test]
fn vpermi2d_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 171, 118, 213], OperandSize::Dword)
}

#[test]
fn vpermi2d_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 174, 118, 60, 87], OperandSize::Dword)
}

#[test]
fn vpermi2d_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 190, 118, 60, 178], OperandSize::Dword)
}

#[test]
fn vpermi2d_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(YMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 50, 61, 162, 118, 223], OperandSize::Qword)
}

#[test]
fn vpermi2d_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 125, 164, 118, 52, 222], OperandSize::Qword)
}

#[test]
fn vpermi2d_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 101, 187, 118, 7], OperandSize::Qword)
}

#[test]
fn vpermi2d_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 206, 118, 230], OperandSize::Dword)
}

#[test]
fn vpermi2d_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Eight, 515505660, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 117, 204, 118, 148, 209, 252, 253, 185, 30], OperandSize::Dword)
}

#[test]
fn vpermi2d_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 771694033, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 101, 222, 118, 164, 193, 209, 29, 255, 45], OperandSize::Dword)
}

#[test]
fn vpermi2d_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 69, 201, 118, 232], OperandSize::Qword)
}

#[test]
fn vpermi2d_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM18)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 109, 194, 118, 6], OperandSize::Qword)
}

#[test]
fn vpermi2d_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 69, 217, 118, 4, 139], OperandSize::Qword)
}

