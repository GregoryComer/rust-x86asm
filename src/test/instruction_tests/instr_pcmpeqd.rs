use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpeqd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 118, 226], OperandSize::Dword)
}

#[test]
fn pcmpeqd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 118, 28, 75], OperandSize::Dword)
}

#[test]
fn pcmpeqd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 118, 248], OperandSize::Qword)
}

#[test]
fn pcmpeqd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(MM1)), operand2: Some(IndirectDisplaced(RBX, 905864580, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 118, 139, 132, 101, 254, 53], OperandSize::Qword)
}

#[test]
fn pcmpeqd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 118, 220], OperandSize::Dword)
}

#[test]
fn pcmpeqd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EBX, 1951021013, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 118, 139, 213, 55, 74, 116], OperandSize::Dword)
}

#[test]
fn pcmpeqd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 118, 254], OperandSize::Qword)
}

#[test]
fn pcmpeqd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Two, 2109278159, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 118, 164, 94, 207, 7, 185, 125], OperandSize::Qword)
}

