use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmulld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 64, 246], OperandSize::Dword)
}

#[test]
fn pmulld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 2134222833, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 64, 60, 181, 241, 167, 53, 127], OperandSize::Dword)
}

#[test]
fn pmulld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 64, 224], OperandSize::Qword)
}

#[test]
fn pmulld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 64, 12, 206], OperandSize::Qword)
}

