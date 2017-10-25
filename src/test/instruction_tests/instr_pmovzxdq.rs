use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovzxdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 53, 238], OperandSize::Dword)
}

#[test]
fn pmovzxdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXDQ, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 53, 1], OperandSize::Dword)
}

#[test]
fn pmovzxdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 53, 200], OperandSize::Qword)
}

#[test]
fn pmovzxdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXDQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RCX, 1880896813, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 53, 169, 45, 53, 28, 112], OperandSize::Qword)
}

