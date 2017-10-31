use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpeqd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 118, 202], OperandSize::Dword)
}

#[test]
fn pcmpeqd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(EBX, 1645759930, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 118, 187, 186, 77, 24, 98], OperandSize::Dword)
}

#[test]
fn pcmpeqd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 118, 210], OperandSize::Qword)
}

#[test]
fn pcmpeqd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(MM0)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 118, 2], OperandSize::Qword)
}

#[test]
fn pcmpeqd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 118, 215], OperandSize::Dword)
}

#[test]
fn pcmpeqd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 118, 44, 192], OperandSize::Dword)
}

#[test]
fn pcmpeqd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 118, 228], OperandSize::Qword)
}

#[test]
fn pcmpeqd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 889574541, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 118, 180, 73, 141, 212, 5, 53], OperandSize::Qword)
}

