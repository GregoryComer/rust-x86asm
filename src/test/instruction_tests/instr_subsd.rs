use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn subsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 92, 250], OperandSize::Dword)
}

#[test]
fn subsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Two, 926421686, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 92, 188, 66, 182, 18, 56, 55], OperandSize::Dword)
}

#[test]
fn subsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 92, 251], OperandSize::Qword)
}

#[test]
fn subsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSD, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 92, 6], OperandSize::Qword)
}

