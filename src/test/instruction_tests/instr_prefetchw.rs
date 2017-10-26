use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn prefetchw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHW, operand1: Some(IndirectDisplaced(EBX, 690401911, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 13, 139, 119, 178, 38, 41], OperandSize::Dword)
}

#[test]
fn prefetchw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHW, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 2078721176, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 13, 140, 75, 152, 196, 230, 123], OperandSize::Qword)
}

