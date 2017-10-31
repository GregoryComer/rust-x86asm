use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsignd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 10, 230], OperandSize::Dword)
}

#[test]
fn vpsignd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 10, 52, 80], OperandSize::Dword)
}

#[test]
fn vpsignd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 10, 208], OperandSize::Qword)
}

#[test]
fn vpsignd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Eight, 1027317350, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 10, 140, 248, 102, 158, 59, 61], OperandSize::Qword)
}

#[test]
fn vpsignd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 10, 228], OperandSize::Dword)
}

#[test]
fn vpsignd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1411567887, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 10, 4, 213, 15, 209, 34, 84], OperandSize::Dword)
}

#[test]
fn vpsignd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 10, 234], OperandSize::Qword)
}

#[test]
fn vpsignd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1929465476, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 10, 44, 69, 132, 78, 1, 115], OperandSize::Qword)
}

