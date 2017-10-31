use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sgdt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SGDT, operand1: Some(IndirectDisplaced(BX, 2768, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 135, 208, 10], OperandSize::Word)
}

#[test]
fn sgdt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SGDT, operand1: Some(IndirectScaledIndexed(EAX, EDX, Two, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 4, 80], OperandSize::Dword)
}

#[test]
fn sgdt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SGDT, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 813798490, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 4, 245, 90, 148, 129, 48], OperandSize::Qword)
}

