use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn frstor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FRSTOR, operand1: Some(IndirectDisplaced(DI, 32212, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 165, 212, 125], OperandSize::Word)
}

#[test]
fn frstor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FRSTOR, operand1: Some(Indirect(ESI, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 38], OperandSize::Dword)
}

#[test]
fn frstor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FRSTOR, operand1: Some(IndirectScaledDisplaced(RCX, Two, 1575874736, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 36, 77, 176, 240, 237, 93], OperandSize::Qword)
}

