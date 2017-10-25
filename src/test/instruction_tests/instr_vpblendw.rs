use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpblendw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 14, 199, 37], OperandSize::Dword)
}

#[test]
fn vpblendw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 636041588, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(58)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 14, 172, 127, 116, 57, 233, 37, 58], OperandSize::Dword)
}

#[test]
fn vpblendw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(66)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 14, 248, 66], OperandSize::Qword)
}

#[test]
fn vpblendw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1818873775, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(105)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 14, 4, 197, 175, 207, 105, 108, 105], OperandSize::Qword)
}

#[test]
fn vpblendw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(104)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 14, 203, 104], OperandSize::Dword)
}

#[test]
fn vpblendw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Four, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(34)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 14, 20, 153, 34], OperandSize::Dword)
}

#[test]
fn vpblendw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(8)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 77, 14, 254, 8], OperandSize::Qword)
}

#[test]
fn vpblendw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(101)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 14, 57, 101], OperandSize::Qword)
}

