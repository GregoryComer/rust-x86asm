use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpblendd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(43)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 2, 254, 43], OperandSize::Dword)
}

#[test]
fn vpblendd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 437927960, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(21)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 2, 156, 215, 24, 64, 26, 26, 21], OperandSize::Dword)
}

#[test]
fn vpblendd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(112)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 2, 213, 112], OperandSize::Qword)
}

#[test]
fn vpblendd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(95)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 2, 36, 120, 95], OperandSize::Qword)
}

#[test]
fn vpblendd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(87)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 2, 227, 87], OperandSize::Dword)
}

#[test]
fn vpblendd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 214486879, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(13)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 2, 20, 69, 95, 207, 200, 12, 13], OperandSize::Dword)
}

#[test]
fn vpblendd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(38)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 2, 238, 38], OperandSize::Qword)
}

#[test]
fn vpblendd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 94472758, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(73)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 77, 2, 36, 85, 54, 138, 161, 5, 73], OperandSize::Qword)
}

