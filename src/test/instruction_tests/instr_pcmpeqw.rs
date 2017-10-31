use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpeqw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 117, 215], OperandSize::Dword)
}

#[test]
fn pcmpeqw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 192213434, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 117, 36, 133, 186, 241, 116, 11], OperandSize::Dword)
}

#[test]
fn pcmpeqw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 117, 227], OperandSize::Qword)
}

#[test]
fn pcmpeqw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 117, 20, 65], OperandSize::Qword)
}

#[test]
fn pcmpeqw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 117, 205], OperandSize::Dword)
}

#[test]
fn pcmpeqw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(EDX, 780718546, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 117, 146, 210, 209, 136, 46], OperandSize::Dword)
}

#[test]
fn pcmpeqw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 117, 243], OperandSize::Qword)
}

#[test]
fn pcmpeqw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 140936179, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 117, 28, 181, 243, 131, 102, 8], OperandSize::Qword)
}

