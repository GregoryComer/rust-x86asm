use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovzxbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 48, 193], OperandSize::Dword)
}

#[test]
fn pmovzxbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EDI, ESI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 48, 12, 183], OperandSize::Dword)
}

#[test]
fn pmovzxbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 48, 195], OperandSize::Qword)
}

#[test]
fn pmovzxbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 48, 60, 142], OperandSize::Qword)
}

