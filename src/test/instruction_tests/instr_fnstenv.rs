use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fnstenv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTENV, operand1: Some(Indirect(BX, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 55], OperandSize::Word)
}

#[test]
fn fnstenv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTENV, operand1: Some(Indirect(EAX, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 48], OperandSize::Dword)
}

#[test]
fn fnstenv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTENV, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 603983788, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 180, 134, 172, 15, 0, 36], OperandSize::Qword)
}

