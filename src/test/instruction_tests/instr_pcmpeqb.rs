use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpeqb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 116, 239], OperandSize::Dword)
}

#[test]
fn pcmpeqb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(MM1)), operand2: Some(IndirectDisplaced(EAX, 552117028, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 116, 136, 36, 163, 232, 32], OperandSize::Dword)
}

#[test]
fn pcmpeqb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 116, 214], OperandSize::Qword)
}

#[test]
fn pcmpeqb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(RAX, 1281150727, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 116, 144, 7, 207, 92, 76], OperandSize::Qword)
}

#[test]
fn pcmpeqb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 116, 222], OperandSize::Dword)
}

#[test]
fn pcmpeqb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(EDX, 1273344021, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 116, 146, 21, 176, 229, 75], OperandSize::Dword)
}

#[test]
fn pcmpeqb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 116, 220], OperandSize::Qword)
}

#[test]
fn pcmpeqb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 116, 34], OperandSize::Qword)
}

