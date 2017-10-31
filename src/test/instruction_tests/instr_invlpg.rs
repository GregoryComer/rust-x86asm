use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn invlpg_1() {
    run_test(&Instruction { mnemonic: Mnemonic::INVLPG, operand1: Some(Memory(22467, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 62, 195, 87], OperandSize::Word)
}

#[test]
fn invlpg_2() {
    run_test(&Instruction { mnemonic: Mnemonic::INVLPG, operand1: Some(IndirectDisplaced(EAX, 1396820663, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 184, 183, 202, 65, 83], OperandSize::Dword)
}

#[test]
fn invlpg_3() {
    run_test(&Instruction { mnemonic: Mnemonic::INVLPG, operand1: Some(IndirectScaledDisplaced(RDI, Four, 956806453, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 60, 189, 53, 181, 7, 57], OperandSize::Qword)
}

