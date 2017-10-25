use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vphaddw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 1, 240], OperandSize::Dword)
}

#[test]
fn vphaddw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 1, 52, 246], OperandSize::Dword)
}

#[test]
fn vphaddw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 1, 244], OperandSize::Qword)
}

#[test]
fn vphaddw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 1, 60, 112], OperandSize::Qword)
}

#[test]
fn vphaddw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 1, 246], OperandSize::Dword)
}

#[test]
fn vphaddw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Eight, 167543596, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 1, 132, 209, 44, 131, 252, 9], OperandSize::Dword)
}

#[test]
fn vphaddw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 1, 211], OperandSize::Qword)
}

#[test]
fn vphaddw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Eight, 163925953, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 1, 172, 222, 193, 79, 197, 9], OperandSize::Qword)
}

