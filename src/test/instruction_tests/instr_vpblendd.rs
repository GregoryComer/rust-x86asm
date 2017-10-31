use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpblendd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(87)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 2, 222, 87], OperandSize::Dword)
}

#[test]
fn vpblendd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 2, 56, 89], OperandSize::Dword)
}

#[test]
fn vpblendd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(77)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 2, 220, 77], OperandSize::Qword)
}

#[test]
fn vpblendd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1399616154, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 2, 12, 189, 154, 114, 108, 83, 59], OperandSize::Qword)
}

#[test]
fn vpblendd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(105)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 2, 225, 105], OperandSize::Dword)
}

#[test]
fn vpblendd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1036827005, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 2, 60, 245, 125, 185, 204, 61, 82], OperandSize::Dword)
}

#[test]
fn vpblendd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 2, 245, 65], OperandSize::Qword)
}

#[test]
fn vpblendd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 1191259950, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(113)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 77, 2, 140, 183, 46, 47, 1, 71, 113], OperandSize::Qword)
}

