use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vhaddps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 124, 225], OperandSize::Dword)
}

#[test]
fn vhaddps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 302549197, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 124, 52, 69, 205, 136, 8, 18], OperandSize::Dword)
}

#[test]
fn vhaddps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 124, 196], OperandSize::Qword)
}

#[test]
fn vhaddps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 570347535, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 124, 28, 181, 15, 208, 254, 33], OperandSize::Qword)
}

#[test]
fn vhaddps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 247, 124, 220], OperandSize::Dword)
}

#[test]
fn vhaddps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 124, 52, 64], OperandSize::Dword)
}

#[test]
fn vhaddps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 215, 124, 242], OperandSize::Qword)
}

#[test]
fn vhaddps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 583946033, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 239, 124, 172, 75, 49, 79, 206, 34], OperandSize::Qword)
}

