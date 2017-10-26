use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovzxdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 53, 251], OperandSize::Dword)
}

#[test]
fn pmovzxdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EAX, 2106904115, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 53, 184, 51, 206, 148, 125], OperandSize::Dword)
}

#[test]
fn pmovzxdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 53, 202], OperandSize::Qword)
}

#[test]
fn pmovzxdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXDQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 68596838, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 53, 172, 128, 102, 180, 22, 4], OperandSize::Qword)
}

