use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpexpandd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 137, 212], OperandSize::Dword)
}

#[test]
fn vpexpandd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 1153110852, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 137, 148, 126, 68, 19, 187, 68], OperandSize::Dword)
}

#[test]
fn vpexpandd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 125, 137, 137, 225], OperandSize::Qword)
}

#[test]
fn vpexpandd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(XMM21)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 266003366, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 125, 140, 137, 172, 201, 166, 227, 218, 15], OperandSize::Qword)
}

#[test]
fn vpexpandd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 137, 205], OperandSize::Dword)
}

#[test]
fn vpexpandd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(EDI, 370606209, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 137, 175, 129, 0, 23, 22], OperandSize::Dword)
}

#[test]
fn vpexpandd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 146, 125, 171, 137, 234], OperandSize::Qword)
}

#[test]
fn vpexpandd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(YMM28)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 511855766, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 125, 169, 137, 164, 118, 150, 76, 130, 30], OperandSize::Qword)
}

#[test]
fn vpexpandd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 137, 239], OperandSize::Dword)
}

#[test]
fn vpexpandd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 178188265, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 137, 36, 253, 233, 239, 158, 10], OperandSize::Dword)
}

#[test]
fn vpexpandd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 18, 125, 202, 137, 219], OperandSize::Qword)
}

#[test]
fn vpexpandd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(ZMM8)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 125, 207, 137, 4, 242], OperandSize::Qword)
}

