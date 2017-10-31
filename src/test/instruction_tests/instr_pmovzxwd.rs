use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovzxwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 51, 216], OperandSize::Dword)
}

#[test]
fn pmovzxwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 51, 16], OperandSize::Dword)
}

#[test]
fn pmovzxwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 51, 226], OperandSize::Qword)
}

#[test]
fn pmovzxwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RDX, 722608847, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 51, 154, 207, 34, 18, 43], OperandSize::Qword)
}

