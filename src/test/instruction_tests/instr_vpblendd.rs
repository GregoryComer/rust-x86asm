use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpblendd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(56)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 2, 232, 56], OperandSize::Dword)
}

#[test]
fn vpblendd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 337087570, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(102)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 2, 12, 245, 82, 140, 23, 20, 102], OperandSize::Dword)
}

#[test]
fn vpblendd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(67)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 2, 198, 67], OperandSize::Qword)
}

#[test]
fn vpblendd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 1376800446, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(18)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 2, 164, 71, 190, 78, 16, 82, 18], OperandSize::Qword)
}

#[test]
fn vpblendd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(106)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 2, 224, 106], OperandSize::Dword)
}

#[test]
fn vpblendd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(124)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 2, 52, 210, 124], OperandSize::Dword)
}

#[test]
fn vpblendd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(57)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 2, 195, 57], OperandSize::Qword)
}

#[test]
fn vpblendd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(126)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 2, 54, 126], OperandSize::Qword)
}

