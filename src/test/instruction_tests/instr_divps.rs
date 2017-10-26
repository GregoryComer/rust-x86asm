use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn divps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 94, 239], OperandSize::Dword)
}

#[test]
fn divps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EAX, 1682208763, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 94, 184, 251, 119, 68, 100], OperandSize::Dword)
}

#[test]
fn divps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 94, 224], OperandSize::Qword)
}

#[test]
fn divps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 94, 52, 154], OperandSize::Qword)
}

