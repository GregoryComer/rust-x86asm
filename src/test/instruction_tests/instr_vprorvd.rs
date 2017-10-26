use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprorvd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 139, 20, 202], OperandSize::Dword)
}

#[test]
fn vprorvd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 139, 20, 20, 129], OperandSize::Dword)
}

#[test]
fn vprorvd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 2012909158, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 85, 159, 20, 20, 157, 102, 142, 250, 119], OperandSize::Dword)
}

#[test]
fn vprorvd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 146, 85, 137, 20, 230], OperandSize::Qword)
}

#[test]
fn vprorvd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Two, 551271538, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 45, 138, 20, 156, 94, 114, 188, 219, 32], OperandSize::Qword)
}

#[test]
fn vprorvd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 151, 20, 12, 126], OperandSize::Qword)
}

#[test]
fn vprorvd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 109, 169, 20, 192], OperandSize::Dword)
}

#[test]
fn vprorvd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Eight, 1970169628, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 175, 20, 140, 200, 28, 103, 110, 117], OperandSize::Dword)
}

#[test]
fn vprorvd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 1344587407, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 188, 20, 148, 119, 143, 198, 36, 80], OperandSize::Dword)
}

#[test]
fn vprorvd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 45, 173, 20, 235], OperandSize::Qword)
}

#[test]
fn vprorvd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Two, 734935886, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 61, 169, 20, 156, 72, 78, 59, 206, 43], OperandSize::Qword)
}

#[test]
fn vprorvd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 1992042020, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 53, 182, 20, 164, 183, 36, 38, 188, 118], OperandSize::Qword)
}

#[test]
fn vprorvd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 20, 236], OperandSize::Dword)
}

#[test]
fn vprorvd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EDI, 1698395909, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 85, 201, 20, 183, 5, 119, 59, 101], OperandSize::Dword)
}

#[test]
fn vprorvd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 219, 20, 28, 89], OperandSize::Dword)
}

#[test]
fn vprorvd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 77, 201, 20, 240], OperandSize::Qword)
}

#[test]
fn vprorvd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 826007013, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 37, 199, 20, 4, 245, 229, 221, 59, 49], OperandSize::Qword)
}

#[test]
fn vprorvd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 85, 212, 20, 4, 115], OperandSize::Qword)
}

