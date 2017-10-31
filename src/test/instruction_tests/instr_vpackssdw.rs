use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpackssdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 107, 236], OperandSize::Dword)
}

#[test]
fn vpackssdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 107, 27], OperandSize::Dword)
}

#[test]
fn vpackssdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 107, 231], OperandSize::Qword)
}

#[test]
fn vpackssdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RDX, 574201896, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 107, 162, 40, 160, 57, 34], OperandSize::Qword)
}

#[test]
fn vpackssdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 107, 218], OperandSize::Dword)
}

#[test]
fn vpackssdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1765361270, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 107, 20, 85, 118, 70, 57, 105], OperandSize::Dword)
}

#[test]
fn vpackssdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 107, 218], OperandSize::Qword)
}

#[test]
fn vpackssdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 918132910, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 107, 140, 202, 174, 152, 185, 54], OperandSize::Qword)
}

#[test]
fn vpackssdw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 143, 107, 242], OperandSize::Dword)
}

#[test]
fn vpackssdw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 693519999, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 143, 107, 52, 157, 127, 70, 86, 41], OperandSize::Dword)
}

#[test]
fn vpackssdw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 69, 156, 107, 60, 71], OperandSize::Dword)
}

#[test]
fn vpackssdw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 5, 138, 107, 218], OperandSize::Qword)
}

#[test]
fn vpackssdw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 870052923, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 93, 131, 107, 28, 205, 59, 244, 219, 51], OperandSize::Qword)
}

#[test]
fn vpackssdw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectDisplaced(RBX, 281710896, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 21, 148, 107, 187, 48, 145, 202, 16], OperandSize::Qword)
}

