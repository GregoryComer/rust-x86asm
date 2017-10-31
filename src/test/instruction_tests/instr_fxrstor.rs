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
    run_test(&Instruction { mnemonic: Mnemonic::FXRSTOR, operand1: Some(IndirectDisplaced(ECX, 1416616152, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 137, 216, 216, 111, 84], OperandSize::Dword)
}

#[test]
fn fxrstor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FXRSTOR, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 151611718, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 12, 245, 70, 105, 9, 9], OperandSize::Qword)
}

