use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sgdt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SGDT, operand1: Some(Indirect(SI, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 4], OperandSize::Word)
}

#[test]
fn sgdt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SGDT, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Eight, 495059530, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 132, 216, 74, 2, 130, 29], OperandSize::Dword)
}

#[test]
fn sgdt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SGDT, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 232866531, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 132, 200, 227, 66, 225, 13], OperandSize::Qword)
}

