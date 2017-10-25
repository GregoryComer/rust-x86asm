use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtps2pi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 227], OperandSize::Word)
}

#[test]
fn cvtps2pi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 236, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 178, 236, 0], OperandSize::Word)
}

#[test]
fn cvtps2pi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 218], OperandSize::Dword)
}

#[test]
fn cvtps2pi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 556011903, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 132, 192, 127, 17, 36, 33], OperandSize::Dword)
}

#[test]
fn cvtps2pi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 243], OperandSize::Qword)
}

#[test]
fn cvtps2pi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM0)), operand2: Some(IndirectDisplaced(RAX, 1356675616, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 128, 32, 58, 221, 80], OperandSize::Qword)
}

