use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn frstor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FRSTOR, operand1: Some(Indirect(DI, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 37], OperandSize::Word)
}

#[test]
fn frstor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FRSTOR, operand1: Some(IndirectScaledIndexed(ECX, EDI, Eight, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 36, 249], OperandSize::Dword)
}

#[test]
fn frstor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FRSTOR, operand1: Some(Indirect(RAX, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 32], OperandSize::Qword)
}

