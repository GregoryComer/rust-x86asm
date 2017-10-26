use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vphsubw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 5, 252], OperandSize::Dword)
}

#[test]
fn vphsubw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDX, 1062343015, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 5, 170, 103, 17, 82, 63], OperandSize::Dword)
}

#[test]
fn vphsubw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 5, 245], OperandSize::Qword)
}

#[test]
fn vphsubw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Two, 2103010359, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 5, 188, 72, 55, 100, 89, 125], OperandSize::Qword)
}

#[test]
fn vphsubw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 5, 222], OperandSize::Dword)
}

#[test]
fn vphsubw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 5, 3], OperandSize::Dword)
}

#[test]
fn vphsubw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 5, 193], OperandSize::Qword)
}

#[test]
fn vphsubw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 1230492025, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 5, 188, 209, 121, 209, 87, 73], OperandSize::Qword)
}

