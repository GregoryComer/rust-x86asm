use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermi2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 141, 119, 207], OperandSize::Dword)
}

#[test]
fn vpermi2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(ECX, 778028376, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 139, 119, 129, 88, 197, 95, 46], OperandSize::Dword)
}

#[test]
fn vpermi2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 710053687, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 85, 153, 119, 12, 213, 55, 143, 82, 42], OperandSize::Dword)
}

#[test]
fn vpermi2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 146, 85, 134, 119, 226], OperandSize::Qword)
}

#[test]
fn vpermi2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectDisplaced(RCX, 1423582309, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 125, 130, 119, 145, 101, 36, 218, 84], OperandSize::Qword)
}

#[test]
fn vpermi2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RAX, 567850850, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 93, 154, 119, 152, 98, 183, 216, 33], OperandSize::Qword)
}

#[test]
fn vpermi2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 175, 119, 201], OperandSize::Dword)
}

#[test]
fn vpermi2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 174, 119, 52, 209], OperandSize::Dword)
}

#[test]
fn vpermi2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 187, 119, 0], OperandSize::Dword)
}

#[test]
fn vpermi2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM8)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 61, 174, 119, 239], OperandSize::Qword)
}

#[test]
fn vpermi2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1022796487, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 69, 165, 119, 60, 93, 199, 162, 246, 60], OperandSize::Qword)
}

#[test]
fn vpermi2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 37, 188, 119, 60, 222], OperandSize::Qword)
}

#[test]
fn vpermi2ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 205, 119, 233], OperandSize::Dword)
}

#[test]
fn vpermi2ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Two, 2052979079, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 85, 201, 119, 132, 65, 135, 249, 93, 122], OperandSize::Dword)
}

#[test]
fn vpermi2ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(EAX, 1371225421, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 220, 119, 168, 77, 61, 187, 81], OperandSize::Dword)
}

#[test]
fn vpermi2ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM30)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 130, 13, 199, 119, 211], OperandSize::Qword)
}

#[test]
fn vpermi2ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectDisplaced(RBX, 1257371785, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 195, 119, 155, 137, 248, 241, 74], OperandSize::Qword)
}

#[test]
fn vpermi2ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Four, 618122099, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 61, 211, 119, 148, 135, 115, 203, 215, 36], OperandSize::Qword)
}

