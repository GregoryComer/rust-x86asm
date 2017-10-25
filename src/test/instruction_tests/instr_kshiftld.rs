use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kshiftld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTLD, operand1: Some(Direct(K5)), operand2: Some(Direct(K7)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 51, 239, 88], OperandSize::Dword)
}

#[test]
fn kshiftld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTLD, operand1: Some(Direct(K3)), operand2: Some(Direct(K7)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 51, 223, 104], OperandSize::Qword)
}

