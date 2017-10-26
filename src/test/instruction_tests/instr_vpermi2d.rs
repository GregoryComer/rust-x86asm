use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermi2d_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 101, 142, 118, 207], OperandSize::Dword)
}

#[test]
fn vpermi2d_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 118, 4, 91], OperandSize::Dword)
}

#[test]
fn vpermi2d_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Two, 646546669, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 109, 159, 118, 164, 70, 237, 132, 137, 38], OperandSize::Dword)
}

#[test]
fn vpermi2d_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 18, 117, 132, 118, 198], OperandSize::Qword)
}

#[test]
fn vpermi2d_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectDisplaced(RDX, 423386128, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 93, 130, 118, 130, 16, 92, 60, 25], OperandSize::Qword)
}

#[test]
fn vpermi2d_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1729559724, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 21, 158, 118, 60, 253, 172, 252, 22, 103], OperandSize::Qword)
}

#[test]
fn vpermi2d_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 175, 118, 239], OperandSize::Dword)
}

#[test]
fn vpermi2d_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 1601496315, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 69, 172, 118, 148, 112, 251, 228, 116, 95], OperandSize::Dword)
}

#[test]
fn vpermi2d_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Two, 1018777617, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 77, 188, 118, 164, 121, 17, 80, 185, 60], OperandSize::Dword)
}

#[test]
fn vpermi2d_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM28)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 194, 29, 167, 118, 192], OperandSize::Qword)
}

#[test]
fn vpermi2d_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM25)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 53, 163, 118, 33], OperandSize::Qword)
}

#[test]
fn vpermi2d_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 45, 185, 118, 52, 81], OperandSize::Qword)
}

#[test]
fn vpermi2d_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 117, 204, 118, 197], OperandSize::Dword)
}

#[test]
fn vpermi2d_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 77, 205, 118, 62], OperandSize::Dword)
}

#[test]
fn vpermi2d_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 1331215779, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 109, 217, 118, 148, 182, 163, 189, 88, 79], OperandSize::Dword)
}

#[test]
fn vpermi2d_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 29, 205, 118, 200], OperandSize::Qword)
}

#[test]
fn vpermi2d_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 925767167, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 53, 196, 118, 60, 245, 255, 21, 46, 55], OperandSize::Qword)
}

#[test]
fn vpermi2d_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 37, 223, 118, 36, 184], OperandSize::Qword)
}

