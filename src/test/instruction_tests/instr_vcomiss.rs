use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcomiss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 47, 224], OperandSize::Dword)
}

#[test]
fn vcomiss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1959881847, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 47, 12, 69, 119, 108, 209, 116], OperandSize::Dword)
}

#[test]
fn vcomiss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 47, 192], OperandSize::Qword)
}

#[test]
fn vcomiss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RDX, 2080995011, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 47, 154, 195, 118, 9, 124], OperandSize::Qword)
}

#[test]
fn vcomiss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 124, 24, 47, 203], OperandSize::Dword)
}

#[test]
fn vcomiss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 757068894, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 47, 4, 205, 94, 244, 31, 45], OperandSize::Dword)
}

#[test]
fn vcomiss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 145, 124, 24, 47, 253], OperandSize::Qword)
}

#[test]
fn vcomiss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM28)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 124, 8, 47, 33], OperandSize::Qword)
}

