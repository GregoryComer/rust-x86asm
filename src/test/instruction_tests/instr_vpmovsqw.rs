use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsqw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 141, 36, 209], OperandSize::Dword)
}

#[test]
fn vpmovsqw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 1096103593, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 36, 180, 215, 169, 54, 85, 65], OperandSize::Dword)
}

#[test]
fn vpmovsqw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 130, 126, 139, 36, 193], OperandSize::Qword)
}

#[test]
fn vpmovsqw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(IndirectScaledDisplaced(RSI, Two, 223064583, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 8, 36, 44, 117, 7, 178, 75, 13], OperandSize::Qword)
}

#[test]
fn vpmovsqw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 175, 36, 220], OperandSize::Dword)
}

#[test]
fn vpmovsqw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(IndirectScaledDisplaced(EDI, Two, 948087322, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 36, 20, 125, 26, 170, 130, 56], OperandSize::Dword)
}

#[test]
fn vpmovsqw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 126, 175, 36, 225], OperandSize::Qword)
}

#[test]
fn vpmovsqw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(IndirectDisplaced(RBX, 96676773, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 40, 36, 155, 165, 43, 195, 5], OperandSize::Qword)
}

#[test]
fn vpmovsqw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 204, 36, 199], OperandSize::Dword)
}

#[test]
fn vpmovsqw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(IndirectScaledDisplaced(EDI, Four, 1693091265, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 36, 52, 189, 193, 133, 234, 100], OperandSize::Dword)
}

#[test]
fn vpmovsqw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM15)), operand2: Some(Direct(ZMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 194, 126, 202, 36, 239], OperandSize::Qword)
}

#[test]
fn vpmovsqw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 620918552, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 36, 188, 67, 24, 119, 2, 37], OperandSize::Qword)
}

