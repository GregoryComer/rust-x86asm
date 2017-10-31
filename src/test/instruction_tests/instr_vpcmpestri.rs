use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpestri_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRI, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 97, 210, 66], OperandSize::Dword)
}

#[test]
fn vpcmpestri_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRI, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 1371976977, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 97, 148, 222, 17, 181, 198, 81, 65], OperandSize::Dword)
}

#[test]
fn vpcmpestri_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRI, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 97, 229, 89], OperandSize::Qword)
}

#[test]
fn vpcmpestri_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRI, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 1695130150, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 97, 188, 186, 38, 162, 9, 101, 32], OperandSize::Qword)
}

