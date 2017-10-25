use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sidt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SIDT, operand1: Some(Indirect(DI, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 13], OperandSize::Word)
}

#[test]
fn sidt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SIDT, operand1: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 12, 130], OperandSize::Dword)
}

#[test]
fn sidt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SIDT, operand1: Some(IndirectScaledDisplaced(RAX, Four, 1988810569, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 12, 133, 73, 215, 138, 118], OperandSize::Qword)
}

