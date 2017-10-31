use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermi2d_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 139, 118, 194], OperandSize::Dword)
}

#[test]
fn vpermi2d_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Two, 83789714, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 137, 118, 180, 123, 146, 135, 254, 4], OperandSize::Dword)
}

#[test]
fn vpermi2d_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 109, 155, 118, 17], OperandSize::Dword)
}

#[test]
fn vpermi2d_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 45, 140, 118, 210], OperandSize::Qword)
}

#[test]
fn vpermi2d_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM23)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 69, 129, 118, 16], OperandSize::Qword)
}

#[test]
fn vpermi2d_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectDisplaced(RDI, 2029554814, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 29, 146, 118, 175, 126, 140, 248, 120], OperandSize::Qword)
}

#[test]
fn vpermi2d_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 175, 118, 194], OperandSize::Dword)
}

#[test]
fn vpermi2d_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 172, 118, 57], OperandSize::Dword)
}

#[test]
fn vpermi2d_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(ECX, 416079150, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 77, 188, 118, 177, 46, 221, 204, 24], OperandSize::Dword)
}

#[test]
fn vpermi2d_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM14)), operand3: Some(Direct(YMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 34, 13, 173, 118, 213], OperandSize::Qword)
}

#[test]
fn vpermi2d_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 77, 163, 118, 28, 115], OperandSize::Qword)
}

#[test]
fn vpermi2d_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Two, 113112204, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 77, 180, 118, 148, 64, 140, 244, 189, 6], OperandSize::Qword)
}

#[test]
fn vpermi2d_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 109, 202, 118, 201], OperandSize::Dword)
}

#[test]
fn vpermi2d_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 203, 118, 36, 158], OperandSize::Dword)
}

#[test]
fn vpermi2d_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(ECX, 1796007173, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 218, 118, 177, 5, 229, 12, 107], OperandSize::Dword)
}

#[test]
fn vpermi2d_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 109, 194, 118, 238], OperandSize::Qword)
}

#[test]
fn vpermi2d_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Eight, 818196136, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 101, 207, 118, 164, 218, 168, 174, 196, 48], OperandSize::Qword)
}

#[test]
fn vpermi2d_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 29337564, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 210, 118, 36, 125, 220, 167, 191, 1], OperandSize::Qword)
}

