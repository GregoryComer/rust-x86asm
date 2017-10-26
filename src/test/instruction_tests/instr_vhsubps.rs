use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vhsubps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 125, 200], OperandSize::Dword)
}

#[test]
fn vhsubps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Eight, 1813754967, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 125, 172, 250, 87, 180, 27, 108], OperandSize::Dword)
}

#[test]
fn vhsubps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 125, 223], OperandSize::Qword)
}

#[test]
fn vhsubps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RBX, 923833052, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 125, 155, 220, 146, 16, 55], OperandSize::Qword)
}

#[test]
fn vhsubps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 215, 125, 194], OperandSize::Dword)
}

#[test]
fn vhsubps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 247, 125, 44, 66], OperandSize::Dword)
}

#[test]
fn vhsubps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 199, 125, 212], OperandSize::Qword)
}

#[test]
fn vhsubps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Eight, 931213435, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 215, 125, 148, 218, 123, 48, 129, 55], OperandSize::Qword)
}

