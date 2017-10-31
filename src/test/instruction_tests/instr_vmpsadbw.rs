use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmpsadbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(98)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 66, 195, 98], OperandSize::Dword)
}

#[test]
fn vmpsadbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Four, 1354530941, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(27)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 66, 188, 155, 125, 128, 188, 80, 27], OperandSize::Dword)
}

#[test]
fn vmpsadbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(75)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 66, 235, 75], OperandSize::Qword)
}

#[test]
fn vmpsadbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RCX, 714944349, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(38)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 66, 153, 93, 47, 157, 42, 38], OperandSize::Qword)
}

#[test]
fn vmpsadbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(11)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 66, 243, 11], OperandSize::Dword)
}

#[test]
fn vmpsadbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(2)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 66, 36, 135, 2], OperandSize::Dword)
}

#[test]
fn vmpsadbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(74)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 66, 223, 74], OperandSize::Qword)
}

#[test]
fn vmpsadbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 2004449986, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(101)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 66, 140, 182, 194, 122, 121, 119, 101], OperandSize::Qword)
}

