use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn aad_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AAD, operand1: Some(Literal8(36)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[213, 36], OperandSize::Word)
}

#[test]
fn aad_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AAD, operand1: Some(Literal8(38)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[213, 38], OperandSize::Dword)
}

