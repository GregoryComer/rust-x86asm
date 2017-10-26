use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rsqrtps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 82, 248], OperandSize::Dword)
}

#[test]
fn rsqrtps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 2001199188, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 82, 52, 205, 84, 224, 71, 119], OperandSize::Dword)
}

#[test]
fn rsqrtps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 82, 204], OperandSize::Qword)
}

#[test]
fn rsqrtps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTPS, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 82, 33], OperandSize::Qword)
}

