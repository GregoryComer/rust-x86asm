use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movups_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 239], OperandSize::Dword)
}

#[test]
fn movups_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 55], OperandSize::Dword)
}

#[test]
fn movups_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 210], OperandSize::Qword)
}

#[test]
fn movups_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 62], OperandSize::Qword)
}

#[test]
fn movups_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 200], OperandSize::Dword)
}

#[test]
fn movups_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(IndirectScaledIndexed(EAX, ESI, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 17, 60, 176], OperandSize::Dword)
}

#[test]
fn movups_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 213], OperandSize::Qword)
}

#[test]
fn movups_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 17, 12, 193], OperandSize::Qword)
}

