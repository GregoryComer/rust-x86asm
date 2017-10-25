use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpeqd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 118, 217], OperandSize::Dword)
}

#[test]
fn pcmpeqd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 739034492, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 118, 4, 117, 124, 197, 12, 44], OperandSize::Dword)
}

#[test]
fn pcmpeqd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 118, 231], OperandSize::Qword)
}

#[test]
fn pcmpeqd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(RSI, 308428681, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 118, 150, 137, 63, 98, 18], OperandSize::Qword)
}

#[test]
fn pcmpeqd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 118, 233], OperandSize::Dword)
}

#[test]
fn pcmpeqd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 911778154, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 118, 132, 207, 106, 161, 88, 54], OperandSize::Dword)
}

#[test]
fn pcmpeqd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 118, 252], OperandSize::Qword)
}

#[test]
fn pcmpeqd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 118, 1], OperandSize::Qword)
}

