use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmulhuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 228, 250], OperandSize::Dword)
}

#[test]
fn pmulhuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Two, 1357618861, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 228, 180, 122, 173, 158, 235, 80], OperandSize::Dword)
}

#[test]
fn pmulhuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 228, 216], OperandSize::Qword)
}

#[test]
fn pmulhuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 569067690, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 228, 132, 143, 170, 72, 235, 33], OperandSize::Qword)
}

#[test]
fn pmulhuw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 228, 222], OperandSize::Dword)
}

#[test]
fn pmulhuw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1028852492, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 228, 44, 125, 12, 11, 83, 61], OperandSize::Dword)
}

#[test]
fn pmulhuw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 228, 201], OperandSize::Qword)
}

#[test]
fn pmulhuw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 228, 51], OperandSize::Qword)
}

