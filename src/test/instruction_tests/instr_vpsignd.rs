use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsignd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 10, 245], OperandSize::Dword)
}

#[test]
fn vpsignd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 10, 28, 88], OperandSize::Dword)
}

#[test]
fn vpsignd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 10, 242], OperandSize::Qword)
}

#[test]
fn vpsignd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 10, 52, 79], OperandSize::Qword)
}

#[test]
fn vpsignd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 10, 199], OperandSize::Dword)
}

#[test]
fn vpsignd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 10, 36, 66], OperandSize::Dword)
}

#[test]
fn vpsignd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 10, 244], OperandSize::Qword)
}

#[test]
fn vpsignd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1650577372, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 10, 20, 221, 220, 207, 97, 98], OperandSize::Qword)
}

