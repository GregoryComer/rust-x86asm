use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kshiftrq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTRQ, operand1: Some(Direct(K1)), operand2: Some(Direct(K6)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 249, 49, 206, 84], OperandSize::Dword)
}

#[test]
fn kshiftrq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTRQ, operand1: Some(Direct(K3)), operand2: Some(Direct(K5)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 249, 49, 221, 87], OperandSize::Qword)
}

