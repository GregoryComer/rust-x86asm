use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xorps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 87, 219], OperandSize::Dword)
}

#[test]
fn xorps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EAX, 47087790, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 87, 128, 174, 128, 206, 2], OperandSize::Dword)
}

#[test]
fn xorps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 87, 195], OperandSize::Qword)
}

#[test]
fn xorps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 558601101, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 87, 172, 88, 141, 147, 75, 33], OperandSize::Qword)
}

