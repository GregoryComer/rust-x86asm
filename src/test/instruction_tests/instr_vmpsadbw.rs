use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmpsadbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(114)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 66, 240, 114], OperandSize::Dword)
}

#[test]
fn vmpsadbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 734602992, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(45)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 66, 164, 79, 240, 38, 201, 43, 45], OperandSize::Dword)
}

#[test]
fn vmpsadbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(100)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 66, 196, 100], OperandSize::Qword)
}

#[test]
fn vmpsadbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(46)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 66, 23, 46], OperandSize::Qword)
}

#[test]
fn vmpsadbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(20)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 66, 197, 20], OperandSize::Dword)
}

#[test]
fn vmpsadbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EAX, 2012693244, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(11)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 66, 136, 252, 66, 247, 119, 11], OperandSize::Dword)
}

#[test]
fn vmpsadbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 66, 247, 108], OperandSize::Qword)
}

#[test]
fn vmpsadbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 66, 16, 0], OperandSize::Qword)
}

