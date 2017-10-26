use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovzxwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 51, 194], OperandSize::Dword)
}

#[test]
fn pmovzxwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Four, 1051383933, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 51, 180, 155, 125, 216, 170, 62], OperandSize::Dword)
}

#[test]
fn pmovzxwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 51, 202], OperandSize::Qword)
}

#[test]
fn pmovzxwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 51, 19], OperandSize::Qword)
}

