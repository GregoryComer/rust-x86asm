use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn minps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 93, 196], OperandSize::Dword)
}

#[test]
fn minps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1862445293, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 93, 4, 77, 237, 168, 2, 111], OperandSize::Dword)
}

#[test]
fn minps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 93, 202], OperandSize::Qword)
}

#[test]
fn minps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 93, 44, 203], OperandSize::Qword)
}

