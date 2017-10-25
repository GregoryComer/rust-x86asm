use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpestri_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRI, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 97, 196, 62], OperandSize::Dword)
}

#[test]
fn vpcmpestri_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRI, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 97, 2, 84], OperandSize::Dword)
}

#[test]
fn vpcmpestri_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRI, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 97, 254, 12], OperandSize::Qword)
}

#[test]
fn vpcmpestri_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRI, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 97, 36, 134, 16], OperandSize::Qword)
}

