use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sidt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SIDT, operand1: Some(IndirectDisplaced(BX, 233, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 143, 233, 0], OperandSize::Word)
}

#[test]
fn sidt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SIDT, operand1: Some(IndirectScaledIndexed(EDI, EDI, Four, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 12, 191], OperandSize::Dword)
}

#[test]
fn sidt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SIDT, operand1: Some(IndirectDisplaced(RBX, 824097197, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 139, 173, 185, 30, 49], OperandSize::Qword)
}

