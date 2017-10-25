use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movntq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTQ, operand1: Some(IndirectDisplaced(DI, 14300, Some(OperandSize::Qword), None)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 231, 133, 220, 55], OperandSize::Word)
}

#[test]
fn movntq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTQ, operand1: Some(IndirectDisplaced(EDX, 1138446899, Some(OperandSize::Qword), None)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 231, 146, 51, 82, 219, 67], OperandSize::Dword)
}

#[test]
fn movntq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTQ, operand1: Some(IndirectScaledDisplaced(RAX, Two, 513320351, Some(OperandSize::Qword), None)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 231, 4, 69, 159, 165, 152, 30], OperandSize::Qword)
}

