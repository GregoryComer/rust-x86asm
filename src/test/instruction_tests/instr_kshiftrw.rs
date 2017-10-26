use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kshiftrw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTRW, operand1: Some(Direct(K3)), operand2: Some(Direct(K7)), operand3: Some(Literal8(39)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 249, 48, 223, 39], OperandSize::Dword)
}

#[test]
fn kshiftrw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTRW, operand1: Some(Direct(K6)), operand2: Some(Direct(K1)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 249, 48, 241, 112], OperandSize::Qword)
}

