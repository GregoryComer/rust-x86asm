use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtps2pi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 211], OperandSize::Word)
}

#[test]
fn cvtps2pi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM6)), operand2: Some(IndirectDisplaced(BX, 162, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 183, 162, 0], OperandSize::Word)
}

#[test]
fn cvtps2pi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 202], OperandSize::Dword)
}

#[test]
fn cvtps2pi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 44, 150], OperandSize::Dword)
}

#[test]
fn cvtps2pi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 203], OperandSize::Qword)
}

#[test]
fn cvtps2pi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 70135283, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 12, 157, 243, 45, 46, 4], OperandSize::Qword)
}

