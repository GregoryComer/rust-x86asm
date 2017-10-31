use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vroundps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(127)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 8, 195, 127], OperandSize::Dword)
}

#[test]
fn vroundps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 8, 60, 198, 35], OperandSize::Dword)
}

#[test]
fn vroundps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 8, 200, 33], OperandSize::Qword)
}

#[test]
fn vroundps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 8, 52, 144, 87], OperandSize::Qword)
}

#[test]
fn vroundps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 8, 240, 114], OperandSize::Dword)
}

#[test]
fn vroundps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Four, 353023914, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(28)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 8, 164, 154, 170, 183, 10, 21, 28], OperandSize::Dword)
}

#[test]
fn vroundps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 8, 253, 96], OperandSize::Qword)
}

#[test]
fn vroundps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(RDI, 1429572237, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 8, 143, 141, 138, 53, 85, 36], OperandSize::Qword)
}

