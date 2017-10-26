use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn invlpg_1() {
    run_test(&Instruction { mnemonic: Mnemonic::INVLPG, operand1: Some(Indirect(SI, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 60], OperandSize::Word)
}

#[test]
fn invlpg_2() {
    run_test(&Instruction { mnemonic: Mnemonic::INVLPG, operand1: Some(IndirectScaledDisplaced(EDI, Four, 985597689, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 60, 189, 249, 6, 191, 58], OperandSize::Dword)
}

#[test]
fn invlpg_3() {
    run_test(&Instruction { mnemonic: Mnemonic::INVLPG, operand1: Some(Indirect(RCX, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 57], OperandSize::Qword)
}

