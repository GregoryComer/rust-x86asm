use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpackssdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 107, 193], OperandSize::Dword)
}

#[test]
fn vpackssdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1031385460, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 107, 36, 77, 116, 177, 121, 61], OperandSize::Dword)
}

#[test]
fn vpackssdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 107, 214], OperandSize::Qword)
}

#[test]
fn vpackssdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RBX, 79142219, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 107, 139, 75, 157, 183, 4], OperandSize::Qword)
}

#[test]
fn vpackssdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 107, 231], OperandSize::Dword)
}

#[test]
fn vpackssdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1737404713, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 107, 12, 253, 41, 177, 142, 103], OperandSize::Dword)
}

#[test]
fn vpackssdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 107, 229], OperandSize::Qword)
}

#[test]
fn vpackssdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 107, 28, 142], OperandSize::Qword)
}

#[test]
fn vpackssdw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 142, 107, 204], OperandSize::Dword)
}

#[test]
fn vpackssdw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 618736844, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 69, 140, 107, 164, 222, 204, 44, 225, 36], OperandSize::Dword)
}

#[test]
fn vpackssdw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 85, 157, 107, 11], OperandSize::Dword)
}

#[test]
fn vpackssdw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 49, 13, 141, 107, 206], OperandSize::Qword)
}

#[test]
fn vpackssdw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 1869037200, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 135, 107, 148, 144, 144, 62, 103, 111], OperandSize::Qword)
}

#[test]
fn vpackssdw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 1903044277, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 5, 151, 107, 164, 95, 181, 38, 110, 113], OperandSize::Qword)
}

