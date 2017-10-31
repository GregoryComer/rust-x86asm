use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovzxwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 51, 196], OperandSize::Dword)
}

#[test]
fn pmovzxwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(ECX, 1417512141, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 51, 145, 205, 132, 125, 84], OperandSize::Dword)
}

#[test]
fn pmovzxwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 51, 198], OperandSize::Qword)
}

#[test]
fn pmovzxwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RBX, 1638873964, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 51, 147, 108, 59, 175, 97], OperandSize::Qword)
}

