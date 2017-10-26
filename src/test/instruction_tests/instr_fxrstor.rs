use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fxrstor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FXRSTOR, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 9], OperandSize::Word)
}

#[test]
fn fxrstor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FXRSTOR, operand1: Some(Indirect(ESI, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 14], OperandSize::Dword)
}

#[test]
fn fxrstor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FXRSTOR, operand1: Some(IndirectDisplaced(RSI, 251034776, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 142, 152, 124, 246, 14], OperandSize::Qword)
}

