use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsqb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 137, 34, 248], OperandSize::Dword)
}

#[test]
fn vpmovsqb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Two, 2137380914, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 34, 180, 123, 50, 216, 101, 127], OperandSize::Dword)
}

#[test]
fn vpmovsqb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 2, 126, 137, 34, 239], OperandSize::Qword)
}

#[test]
fn vpmovsqb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(IndirectDisplaced(RDI, 1827431776, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 8, 34, 143, 96, 101, 236, 108], OperandSize::Qword)
}

#[test]
fn vpmovsqb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 171, 34, 205], OperandSize::Dword)
}

#[test]
fn vpmovsqb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(IndirectDisplaced(ESI, 784028471, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 34, 150, 55, 83, 187, 46], OperandSize::Dword)
}

#[test]
fn vpmovsqb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 126, 171, 34, 225], OperandSize::Qword)
}

#[test]
fn vpmovsqb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 85706930, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 34, 164, 118, 178, 200, 27, 5], OperandSize::Qword)
}

#[test]
fn vpmovsqb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 206, 34, 249], OperandSize::Dword)
}

#[test]
fn vpmovsqb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(IndirectScaledIndexed(EDX, ESI, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 34, 60, 114], OperandSize::Dword)
}

#[test]
fn vpmovsqb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM21)), operand2: Some(Direct(ZMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 50, 126, 202, 34, 221], OperandSize::Qword)
}

#[test]
fn vpmovsqb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 1423754857, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 34, 60, 221, 105, 198, 220, 84], OperandSize::Qword)
}

