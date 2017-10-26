use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpeqd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 118, 222], OperandSize::Dword)
}

#[test]
fn pcmpeqd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 973307105, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 118, 28, 69, 225, 124, 3, 58], OperandSize::Dword)
}

#[test]
fn pcmpeqd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 118, 242], OperandSize::Qword)
}

#[test]
fn pcmpeqd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 981339485, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 118, 60, 213, 93, 13, 126, 58], OperandSize::Qword)
}

#[test]
fn pcmpeqd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 118, 208], OperandSize::Dword)
}

#[test]
fn pcmpeqd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 118, 51], OperandSize::Dword)
}

#[test]
fn pcmpeqd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 118, 219], OperandSize::Qword)
}

#[test]
fn pcmpeqd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RDX, RDI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 118, 20, 122], OperandSize::Qword)
}

