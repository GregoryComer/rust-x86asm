use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpeqb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 116, 210], OperandSize::Dword)
}

#[test]
fn pcmpeqb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(MM1)), operand2: Some(IndirectDisplaced(EBX, 1729386136, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 116, 139, 152, 86, 20, 103], OperandSize::Dword)
}

#[test]
fn pcmpeqb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 116, 225], OperandSize::Qword)
}

#[test]
fn pcmpeqb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Two, 841429359, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 116, 164, 122, 111, 49, 39, 50], OperandSize::Qword)
}

#[test]
fn pcmpeqb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 116, 230], OperandSize::Dword)
}

#[test]
fn pcmpeqb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 116, 20, 86], OperandSize::Dword)
}

#[test]
fn pcmpeqb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 116, 228], OperandSize::Qword)
}

#[test]
fn pcmpeqb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 116, 9], OperandSize::Qword)
}

