use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rcpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 83, 195], OperandSize::Dword)
}

#[test]
fn rcpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Four, 133464866, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 83, 156, 139, 34, 131, 244, 7], OperandSize::Dword)
}

#[test]
fn rcpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 83, 211], OperandSize::Qword)
}

#[test]
fn rcpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 83, 44, 74], OperandSize::Qword)
}

