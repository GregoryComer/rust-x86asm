use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kshiftrd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTRD, operand1: Some(Direct(K1)), operand2: Some(Direct(K3)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 49, 203, 43], OperandSize::Dword)
}

#[test]
fn kshiftrd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTRD, operand1: Some(Direct(K2)), operand2: Some(Direct(K6)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 49, 214, 43], OperandSize::Qword)
}

