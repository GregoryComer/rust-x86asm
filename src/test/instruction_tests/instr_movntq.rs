use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movntq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTQ, operand1: Some(IndirectDisplaced(BP, 16097, Some(OperandSize::Qword), None)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 231, 142, 225, 62], OperandSize::Word)
}

#[test]
fn movntq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTQ, operand1: Some(IndirectDisplaced(ESI, 799456014, Some(OperandSize::Qword), None)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 231, 174, 14, 187, 166, 47], OperandSize::Dword)
}

#[test]
fn movntq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTQ, operand1: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 231, 60, 219], OperandSize::Qword)
}

