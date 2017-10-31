use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vroundps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 8, 236, 5], OperandSize::Dword)
}

#[test]
fn vroundps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 372149487, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 8, 12, 189, 239, 140, 46, 22, 54], OperandSize::Dword)
}

#[test]
fn vroundps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 8, 193, 94], OperandSize::Qword)
}

#[test]
fn vroundps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 337714402, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(18)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 8, 188, 139, 226, 28, 33, 20, 18], OperandSize::Qword)
}

#[test]
fn vroundps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 8, 206, 82], OperandSize::Dword)
}

#[test]
fn vroundps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 8, 12, 194, 51], OperandSize::Dword)
}

#[test]
fn vroundps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(20)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 8, 199, 20], OperandSize::Qword)
}

#[test]
fn vroundps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 67536535, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 8, 20, 93, 151, 134, 6, 4, 60], OperandSize::Qword)
}

