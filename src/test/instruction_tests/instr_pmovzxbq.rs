use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovzxbq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 50, 206], OperandSize::Dword)
}

#[test]
fn pmovzxbq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 1569677621, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 50, 52, 141, 53, 97, 143, 93], OperandSize::Dword)
}

#[test]
fn pmovzxbq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 50, 233], OperandSize::Qword)
}

#[test]
fn pmovzxbq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 2119835452, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 50, 36, 213, 60, 31, 90, 126], OperandSize::Qword)
}

