use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovzxwq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 52, 198], OperandSize::Dword)
}

#[test]
fn pmovzxwq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 52, 60, 75], OperandSize::Dword)
}

#[test]
fn pmovzxwq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 52, 238], OperandSize::Qword)
}

#[test]
fn pmovzxwq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWQ, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 52, 6], OperandSize::Qword)
}

