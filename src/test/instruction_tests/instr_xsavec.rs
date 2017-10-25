use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xsavec_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVEC, operand1: Some(IndirectDisplaced(SI, 1046, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 164, 22, 4], OperandSize::Word)
}

#[test]
fn xsavec_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVEC, operand1: Some(Indirect(EAX, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 32], OperandSize::Dword)
}

#[test]
fn xsavec_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVEC, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Eight, 2094586999, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 164, 198, 119, 220, 216, 124], OperandSize::Qword)
}

