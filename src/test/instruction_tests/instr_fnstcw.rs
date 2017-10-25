use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fnstcw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTCW, operand1: Some(Memory(28522, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 62, 106, 111], OperandSize::Word)
}

#[test]
fn fnstcw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTCW, operand1: Some(Indirect(ECX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 57], OperandSize::Dword)
}

#[test]
fn fnstcw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTCW, operand1: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 60, 182], OperandSize::Qword)
}

