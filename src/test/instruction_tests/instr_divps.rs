use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn divps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 94, 233], OperandSize::Dword)
}

#[test]
fn divps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EAX, 1687310625, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 94, 136, 33, 81, 146, 100], OperandSize::Dword)
}

#[test]
fn divps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 94, 218], OperandSize::Qword)
}

#[test]
fn divps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RCX, 1769987443, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 94, 185, 115, 221, 127, 105], OperandSize::Qword)
}

