use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn aesenclast_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENCLAST, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 221, 232], OperandSize::Dword)
}

#[test]
fn aesenclast_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENCLAST, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 821397022, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 221, 20, 69, 30, 134, 245, 48], OperandSize::Dword)
}

#[test]
fn aesenclast_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENCLAST, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 221, 230], OperandSize::Qword)
}

#[test]
fn aesenclast_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENCLAST, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 221, 12, 190], OperandSize::Qword)
}

