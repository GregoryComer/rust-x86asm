use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rcpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 83, 203], OperandSize::Dword)
}

#[test]
fn rcpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 83, 44, 211], OperandSize::Dword)
}

#[test]
fn rcpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 83, 239], OperandSize::Qword)
}

#[test]
fn rcpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 1402910534, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 83, 60, 253, 70, 183, 158, 83], OperandSize::Qword)
}

