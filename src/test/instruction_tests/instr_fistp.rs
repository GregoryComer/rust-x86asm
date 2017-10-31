use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fistp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 26], OperandSize::Word)
}

#[test]
fn fistp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 24], OperandSize::Dword)
}

#[test]
fn fistp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectDisplaced(RAX, 416612692, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 152, 84, 1, 213, 24], OperandSize::Qword)
}

#[test]
fn fistp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 26], OperandSize::Word)
}

#[test]
fn fistp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 28, 130], OperandSize::Dword)
}

#[test]
fn fistp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 28, 201], OperandSize::Qword)
}

#[test]
fn fistp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectDisplaced(DI, 186, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 189, 186, 0], OperandSize::Word)
}

#[test]
fn fistp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectDisplaced(ECX, 1484123268, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 185, 132, 236, 117, 88], OperandSize::Dword)
}

#[test]
fn fistp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 60, 211], OperandSize::Qword)
}

