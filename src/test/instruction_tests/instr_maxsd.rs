use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn maxsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 95, 218], OperandSize::Dword)
}

#[test]
fn maxsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Two, 1743433425, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 95, 148, 122, 209, 174, 234, 103], OperandSize::Dword)
}

#[test]
fn maxsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 95, 253], OperandSize::Qword)
}

#[test]
fn maxsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 95, 4, 144], OperandSize::Qword)
}

