use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn divps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 94, 241], OperandSize::Dword)
}

#[test]
fn divps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EDX, 1772407505, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 94, 130, 209, 202, 164, 105], OperandSize::Dword)
}

#[test]
fn divps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 94, 243], OperandSize::Qword)
}

#[test]
fn divps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 1977142426, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 94, 156, 82, 154, 204, 216, 117], OperandSize::Qword)
}

