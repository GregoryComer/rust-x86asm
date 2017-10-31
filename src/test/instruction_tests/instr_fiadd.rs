use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fiadd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 7], OperandSize::Word)
}

#[test]
fn fiadd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(IndirectDisplaced(EAX, 313653072, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 128, 80, 247, 177, 18], OperandSize::Dword)
}

#[test]
fn fiadd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 1], OperandSize::Qword)
}

#[test]
fn fiadd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 24650, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 130, 74, 96], OperandSize::Word)
}

#[test]
fn fiadd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(IndirectDisplaced(EDX, 1953835327, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 130, 63, 41, 117, 116], OperandSize::Dword)
}

#[test]
fn fiadd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 164741419, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 132, 87, 43, 193, 209, 9], OperandSize::Qword)
}

