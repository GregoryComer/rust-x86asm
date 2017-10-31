use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movq2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVQ2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 214, 196], OperandSize::Word)
}

#[test]
fn movq2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVQ2DQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 214, 212], OperandSize::Dword)
}

#[test]
fn movq2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVQ2DQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 214, 204], OperandSize::Qword)
}

