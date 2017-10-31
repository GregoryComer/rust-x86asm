use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rcpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 83, 222], OperandSize::Dword)
}

#[test]
fn rcpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1312094247, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 83, 28, 253, 39, 248, 52, 78], OperandSize::Dword)
}

#[test]
fn rcpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 83, 216], OperandSize::Qword)
}

#[test]
fn rcpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 83, 60, 66], OperandSize::Qword)
}

