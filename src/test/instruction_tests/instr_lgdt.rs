use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lgdt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LGDT, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 18], OperandSize::Word)
}

#[test]
fn lgdt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LGDT, operand1: Some(IndirectDisplaced(EBX, 332019481, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 147, 25, 55, 202, 19], OperandSize::Dword)
}

#[test]
fn lgdt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LGDT, operand1: Some(Indirect(RAX, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 16], OperandSize::Qword)
}

