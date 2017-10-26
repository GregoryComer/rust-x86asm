use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fnstenv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTENV, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 50], OperandSize::Word)
}

#[test]
fn fnstenv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTENV, operand1: Some(Indirect(ECX, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 49], OperandSize::Dword)
}

#[test]
fn fnstenv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTENV, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 867710737, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 52, 245, 17, 55, 184, 51], OperandSize::Qword)
}

