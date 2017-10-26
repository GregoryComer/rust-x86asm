use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kandd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KANDD, operand1: Some(Direct(K4)), operand2: Some(Direct(K3)), operand3: Some(Direct(K7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 229, 65, 231], OperandSize::Dword)
}

#[test]
fn kandd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KANDD, operand1: Some(Direct(K7)), operand2: Some(Direct(K4)), operand3: Some(Direct(K1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 221, 65, 249], OperandSize::Qword)
}

