use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movntq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTQ, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 19, Some(OperandSize::Qword), None)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 231, 120, 19], OperandSize::Word)
}

#[test]
fn movntq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTQ, operand1: Some(IndirectDisplaced(EDI, 1703226337, Some(OperandSize::Qword), None)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 231, 135, 225, 43, 133, 101], OperandSize::Dword)
}

#[test]
fn movntq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTQ, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 1092804910, Some(OperandSize::Qword), None)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 231, 140, 70, 46, 225, 34, 65], OperandSize::Qword)
}

