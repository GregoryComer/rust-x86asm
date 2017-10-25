use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermi2d_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 139, 118, 228], OperandSize::Dword)
}

#[test]
fn vpermi2d_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 117, 141, 118, 38], OperandSize::Dword)
}

#[test]
fn vpermi2d_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 109, 158, 118, 42], OperandSize::Dword)
}

#[test]
fn vpermi2d_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 135, 118, 200], OperandSize::Qword)
}

#[test]
fn vpermi2d_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 220668113, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 29, 143, 118, 140, 138, 209, 32, 39, 13], OperandSize::Qword)
}

#[test]
fn vpermi2d_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1702339053, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 29, 158, 118, 36, 157, 237, 161, 119, 101], OperandSize::Qword)
}

#[test]
fn vpermi2d_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 85, 169, 118, 251], OperandSize::Dword)
}

#[test]
fn vpermi2d_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 117, 170, 118, 52, 128], OperandSize::Dword)
}

#[test]
fn vpermi2d_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1583799256, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 185, 118, 12, 157, 216, 219, 102, 94], OperandSize::Dword)
}

#[test]
fn vpermi2d_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 2, 117, 174, 118, 208], OperandSize::Qword)
}

#[test]
fn vpermi2d_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Two, 295838360, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 69, 169, 118, 140, 94, 152, 34, 162, 17], OperandSize::Qword)
}

#[test]
fn vpermi2d_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 47835787, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 101, 180, 118, 4, 245, 139, 234, 217, 2], OperandSize::Qword)
}

#[test]
fn vpermi2d_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 118, 233], OperandSize::Dword)
}

#[test]
fn vpermi2d_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 207, 118, 39], OperandSize::Dword)
}

#[test]
fn vpermi2d_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 117, 218, 118, 28, 203], OperandSize::Dword)
}

#[test]
fn vpermi2d_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(ZMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 50, 29, 196, 118, 246], OperandSize::Qword)
}

#[test]
fn vpermi2d_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Two, 86995973, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 13, 198, 118, 180, 91, 5, 116, 47, 5], OperandSize::Qword)
}

#[test]
fn vpermi2d_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 928574620, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 13, 217, 118, 148, 153, 156, 236, 88, 55], OperandSize::Qword)
}

