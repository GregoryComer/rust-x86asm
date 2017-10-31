use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kshiftld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTLD, operand1: Some(Direct(K6)), operand2: Some(Direct(K3)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 51, 243, 126], OperandSize::Dword)
}

#[test]
fn kshiftld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTLD, operand1: Some(Direct(K1)), operand2: Some(Direct(K5)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 51, 205, 9], OperandSize::Qword)
}

