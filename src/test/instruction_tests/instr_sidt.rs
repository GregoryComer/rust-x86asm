use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sidt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SIDT, operand1: Some(IndirectDisplaced(SI, 110, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 76, 110], OperandSize::Word)
}

#[test]
fn sidt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SIDT, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Four, 1645513045, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 140, 153, 85, 137, 20, 98], OperandSize::Dword)
}

#[test]
fn sidt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SIDT, operand1: Some(Indirect(RDI, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 15], OperandSize::Qword)
}

