use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fldenv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FLDENV, operand1: Some(IndirectDisplaced(DI, 27157, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 165, 21, 106], OperandSize::Word)
}

#[test]
fn fldenv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FLDENV, operand1: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Eight, 113667832, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 164, 240, 248, 110, 198, 6], OperandSize::Dword)
}

#[test]
fn fldenv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FLDENV, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 977287690, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 36, 245, 10, 58, 64, 58], OperandSize::Qword)
}

