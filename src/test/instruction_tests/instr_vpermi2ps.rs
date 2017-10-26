use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermi2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 139, 119, 212], OperandSize::Dword)
}

#[test]
fn vpermi2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EAX, 791413656, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 139, 119, 128, 152, 3, 44, 47], OperandSize::Dword)
}

#[test]
fn vpermi2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 117, 154, 119, 12, 240], OperandSize::Dword)
}

#[test]
fn vpermi2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 162, 85, 138, 119, 206], OperandSize::Qword)
}

#[test]
fn vpermi2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RBX, 578555843, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 77, 143, 119, 139, 195, 15, 124, 34], OperandSize::Qword)
}

#[test]
fn vpermi2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Eight, 714649229, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 61, 157, 119, 164, 255, 141, 174, 152, 42], OperandSize::Qword)
}

#[test]
fn vpermi2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 174, 119, 234], OperandSize::Dword)
}

#[test]
fn vpermi2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 175, 119, 55], OperandSize::Dword)
}

#[test]
fn vpermi2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 85, 191, 119, 43], OperandSize::Dword)
}

#[test]
fn vpermi2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 34, 37, 172, 119, 242], OperandSize::Qword)
}

#[test]
fn vpermi2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 101, 174, 119, 12, 248], OperandSize::Qword)
}

#[test]
fn vpermi2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 77, 187, 119, 3], OperandSize::Qword)
}

#[test]
fn vpermi2ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 119, 220], OperandSize::Dword)
}

#[test]
fn vpermi2ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 207, 119, 60, 90], OperandSize::Dword)
}

#[test]
fn vpermi2ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EAX, 1072562387, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 218, 119, 136, 211, 0, 238, 63], OperandSize::Dword)
}

#[test]
fn vpermi2ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 109, 197, 119, 224], OperandSize::Qword)
}

#[test]
fn vpermi2ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectDisplaced(RCX, 861985334, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 53, 207, 119, 161, 54, 218, 96, 51], OperandSize::Qword)
}

#[test]
fn vpermi2ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 125, 222, 119, 52, 131], OperandSize::Qword)
}

