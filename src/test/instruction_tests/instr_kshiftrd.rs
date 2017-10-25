use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kshiftrd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTRD, operand1: Some(Direct(K3)), operand2: Some(Direct(K7)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 49, 223, 50], OperandSize::Dword)
}

#[test]
fn kshiftrd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTRD, operand1: Some(Direct(K5)), operand2: Some(Direct(K4)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 49, 236, 114], OperandSize::Qword)
}

