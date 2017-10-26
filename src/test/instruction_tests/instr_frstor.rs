use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn frstor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FRSTOR, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 35], OperandSize::Word)
}

#[test]
fn frstor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FRSTOR, operand1: Some(IndirectDisplaced(EDX, 61894511, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 162, 111, 111, 176, 3], OperandSize::Dword)
}

#[test]
fn frstor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FRSTOR, operand1: Some(Indirect(RCX, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 33], OperandSize::Qword)
}

