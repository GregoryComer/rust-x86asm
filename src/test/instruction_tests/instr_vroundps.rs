use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vroundps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(23)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 8, 195, 23], OperandSize::Dword)
}

#[test]
fn vroundps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 8, 36, 243, 96], OperandSize::Dword)
}

#[test]
fn vroundps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(11)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 8, 224, 11], OperandSize::Qword)
}

#[test]
fn vroundps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(127)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 8, 36, 126, 127], OperandSize::Qword)
}

#[test]
fn vroundps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 8, 201, 56], OperandSize::Dword)
}

#[test]
fn vroundps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 8, 16, 24], OperandSize::Dword)
}

#[test]
fn vroundps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 8, 208, 32], OperandSize::Qword)
}

#[test]
fn vroundps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 8, 48, 43], OperandSize::Qword)
}

