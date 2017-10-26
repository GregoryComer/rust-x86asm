use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvttps2pi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 233], OperandSize::Word)
}

#[test]
fn cvttps2pi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 97, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 90, 97], OperandSize::Word)
}

#[test]
fn cvttps2pi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 225], OperandSize::Dword)
}

#[test]
fn cvttps2pi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM3)), operand2: Some(IndirectDisplaced(EBX, 859025426, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 155, 18, 176, 51, 51], OperandSize::Dword)
}

#[test]
fn cvttps2pi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 201], OperandSize::Qword)
}

#[test]
fn cvttps2pi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 106087853, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 60, 149, 173, 197, 82, 6], OperandSize::Qword)
}

