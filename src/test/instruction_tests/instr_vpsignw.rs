use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsignw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 9, 237], OperandSize::Dword)
}

#[test]
fn vpsignw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ECX, 379396935, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 9, 169, 71, 35, 157, 22], OperandSize::Dword)
}

#[test]
fn vpsignw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 9, 234], OperandSize::Qword)
}

#[test]
fn vpsignw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 9, 28, 203], OperandSize::Qword)
}

#[test]
fn vpsignw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 9, 205], OperandSize::Dword)
}

#[test]
fn vpsignw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 9, 50], OperandSize::Dword)
}

#[test]
fn vpsignw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 9, 235], OperandSize::Qword)
}

#[test]
fn vpsignw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 9, 3], OperandSize::Qword)
}

