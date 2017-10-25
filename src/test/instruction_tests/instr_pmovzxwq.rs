use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovzxwq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 52, 235], OperandSize::Dword)
}

#[test]
fn pmovzxwq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 418426966, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 52, 4, 157, 86, 176, 240, 24], OperandSize::Dword)
}

#[test]
fn pmovzxwq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 52, 239], OperandSize::Qword)
}

#[test]
fn pmovzxwq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 52, 60, 178], OperandSize::Qword)
}

