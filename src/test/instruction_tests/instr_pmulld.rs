use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmulld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 64, 252], OperandSize::Dword)
}

#[test]
fn pmulld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 64, 4, 216], OperandSize::Dword)
}

#[test]
fn pmulld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 64, 206], OperandSize::Qword)
}

#[test]
fn pmulld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RSI, 256904940, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 64, 134, 236, 14, 80, 15], OperandSize::Qword)
}

