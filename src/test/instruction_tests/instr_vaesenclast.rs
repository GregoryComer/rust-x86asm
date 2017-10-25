use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaesenclast_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENCLAST, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 221, 235], OperandSize::Dword)
}

#[test]
fn vaesenclast_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENCLAST, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EAX, 2101033480, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 221, 160, 8, 58, 59, 125], OperandSize::Dword)
}

#[test]
fn vaesenclast_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENCLAST, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 221, 246], OperandSize::Qword)
}

#[test]
fn vaesenclast_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENCLAST, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Two, 314298895, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 221, 164, 123, 15, 210, 187, 18], OperandSize::Qword)
}

