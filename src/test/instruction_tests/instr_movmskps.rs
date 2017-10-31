use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movmskps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVMSKPS, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 80, 221], OperandSize::Dword)
}

#[test]
fn movmskps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVMSKPS, operand1: Some(Direct(RBP)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 80, 234], OperandSize::Qword)
}

