use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn aad_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AAD, operand1: Some(Literal8(30)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[213, 30], OperandSize::Word)
}

#[test]
fn aad_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AAD, operand1: Some(Literal8(61)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[213, 61], OperandSize::Dword)
}

