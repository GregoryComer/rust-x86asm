use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fnsave_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSAVE, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 49], OperandSize::Word)
}

#[test]
fn fnsave_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSAVE, operand1: Some(IndirectScaledDisplaced(ESI, Eight, 1842864009, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 52, 245, 137, 223, 215, 109], OperandSize::Dword)
}

#[test]
fn fnsave_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSAVE, operand1: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 52, 240], OperandSize::Qword)
}

