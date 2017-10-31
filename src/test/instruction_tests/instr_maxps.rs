use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn maxps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 95, 233], OperandSize::Dword)
}

#[test]
fn maxps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 1691229776, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 95, 28, 149, 80, 30, 206, 100], OperandSize::Dword)
}

#[test]
fn maxps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 95, 229], OperandSize::Qword)
}

#[test]
fn maxps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 95, 4, 87], OperandSize::Qword)
}

