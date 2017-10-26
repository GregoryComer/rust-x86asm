use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sgdt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SGDT, operand1: Some(Memory(7942, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 6, 6, 31], OperandSize::Word)
}

#[test]
fn sgdt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SGDT, operand1: Some(IndirectDisplaced(ECX, 1599724274, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 129, 242, 218, 89, 95], OperandSize::Dword)
}

#[test]
fn sgdt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SGDT, operand1: Some(IndirectDisplaced(RAX, 581580906, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 128, 106, 56, 170, 34], OperandSize::Qword)
}

