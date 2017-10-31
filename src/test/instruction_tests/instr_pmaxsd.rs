use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaxsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 61, 197], OperandSize::Dword)
}

#[test]
fn pmaxsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Two, 1539752350, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 61, 164, 94, 158, 193, 198, 91], OperandSize::Dword)
}

#[test]
fn pmaxsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 61, 203], OperandSize::Qword)
}

#[test]
fn pmaxsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 415308552, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 61, 148, 182, 8, 27, 193, 24], OperandSize::Qword)
}

