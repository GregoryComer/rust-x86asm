use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpeqb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 116, 241], OperandSize::Dword)
}

#[test]
fn pcmpeqb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(MM1)), operand2: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 116, 11], OperandSize::Dword)
}

#[test]
fn pcmpeqb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 116, 237], OperandSize::Qword)
}

#[test]
fn pcmpeqb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 116, 52, 142], OperandSize::Qword)
}

#[test]
fn pcmpeqb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 116, 208], OperandSize::Dword)
}

#[test]
fn pcmpeqb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(ESI, 984993813, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 116, 158, 21, 208, 181, 58], OperandSize::Dword)
}

#[test]
fn pcmpeqb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 116, 201], OperandSize::Qword)
}

#[test]
fn pcmpeqb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RAX, 1261692884, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 116, 176, 212, 231, 51, 75], OperandSize::Qword)
}

