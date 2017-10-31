use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kshiftlb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTLB, operand1: Some(Direct(K3)), operand2: Some(Direct(K5)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 50, 221, 16], OperandSize::Dword)
}

#[test]
fn kshiftlb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTLB, operand1: Some(Direct(K1)), operand2: Some(Direct(K7)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 50, 207, 58], OperandSize::Qword)
}

