use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn aam_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AAM, operand1: Some(Literal8(24)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[212, 24], OperandSize::Word)
}

#[test]
fn aam_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AAM, operand1: Some(Literal8(49)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[212, 49], OperandSize::Dword)
}

