use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kxorq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KXORQ, operand1: Some(Direct(K6)), operand2: Some(Direct(K3)), operand3: Some(Direct(K4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 228, 71, 244], OperandSize::Dword)
}

#[test]
fn kxorq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KXORQ, operand1: Some(Direct(K2)), operand2: Some(Direct(K3)), operand3: Some(Direct(K7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 228, 71, 215], OperandSize::Qword)
}

