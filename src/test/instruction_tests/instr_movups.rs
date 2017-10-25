use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movups_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 246], OperandSize::Dword)
}

#[test]
fn movups_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 2], OperandSize::Dword)
}

#[test]
fn movups_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 198], OperandSize::Qword)
}

#[test]
fn movups_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RDI, RDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 4, 191], OperandSize::Qword)
}

#[test]
fn movups_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 220], OperandSize::Dword)
}

#[test]
fn movups_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 17, 34], OperandSize::Dword)
}

#[test]
fn movups_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 225], OperandSize::Qword)
}

#[test]
fn movups_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 17, 7], OperandSize::Qword)
}

