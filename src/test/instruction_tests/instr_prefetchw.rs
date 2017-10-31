use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn prefetchw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHW, operand1: Some(Indirect(EDI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 13, 15], OperandSize::Dword)
}

#[test]
fn prefetchw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHW, operand1: Some(IndirectDisplaced(RAX, 861358125, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 13, 136, 45, 72, 87, 51], OperandSize::Qword)
}

