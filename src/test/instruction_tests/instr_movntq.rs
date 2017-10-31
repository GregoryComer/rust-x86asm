use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movntq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTQ, operand1: Some(IndirectDisplaced(BP, 6318, Some(OperandSize::Qword), None)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 231, 174, 174, 24], OperandSize::Word)
}

#[test]
fn movntq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTQ, operand1: Some(IndirectScaledDisplaced(EAX, Two, 460573418, Some(OperandSize::Qword), None)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 231, 36, 69, 234, 202, 115, 27], OperandSize::Dword)
}

#[test]
fn movntq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTQ, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 1227190060, Some(OperandSize::Qword), None)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 231, 156, 113, 44, 111, 37, 73], OperandSize::Qword)
}

