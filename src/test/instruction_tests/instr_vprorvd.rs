use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprorvd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 143, 20, 230], OperandSize::Dword)
}

#[test]
fn vprorvd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 69, 138, 20, 4, 79], OperandSize::Dword)
}

#[test]
fn vprorvd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 77, 157, 20, 28, 86], OperandSize::Dword)
}

#[test]
fn vprorvd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 34, 13, 129, 20, 212], OperandSize::Qword)
}

#[test]
fn vprorvd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectDisplaced(RAX, 235421630, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 29, 129, 20, 136, 190, 63, 8, 14], OperandSize::Qword)
}

#[test]
fn vprorvd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectDisplaced(RDI, 268075784, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 53, 157, 20, 159, 8, 131, 250, 15], OperandSize::Qword)
}

#[test]
fn vprorvd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 20, 198], OperandSize::Dword)
}

#[test]
fn vprorvd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 172, 20, 26], OperandSize::Dword)
}

#[test]
fn vprorvd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1325855203, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 77, 189, 20, 12, 77, 227, 241, 6, 79], OperandSize::Dword)
}

#[test]
fn vprorvd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 178, 21, 175, 20, 218], OperandSize::Qword)
}

#[test]
fn vprorvd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM24)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 61, 166, 20, 47], OperandSize::Qword)
}

#[test]
fn vprorvd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 101, 189, 20, 47], OperandSize::Qword)
}

#[test]
fn vprorvd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 203, 20, 206], OperandSize::Dword)
}

#[test]
fn vprorvd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EDX, 86614317, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 205, 20, 154, 45, 161, 41, 5], OperandSize::Dword)
}

#[test]
fn vprorvd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(ECX, 1133783376, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 101, 220, 20, 137, 80, 41, 148, 67], OperandSize::Dword)
}

#[test]
fn vprorvd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM14)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 13, 201, 20, 220], OperandSize::Qword)
}

#[test]
fn vprorvd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectDisplaced(RSI, 216478671, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 29, 195, 20, 166, 207, 51, 231, 12], OperandSize::Qword)
}

#[test]
fn vprorvd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectDisplaced(RSI, 963003739, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 5, 221, 20, 174, 91, 69, 102, 57], OperandSize::Qword)
}

