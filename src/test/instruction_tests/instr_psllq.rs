use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psllq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM3)), operand2: Some(Literal8(50)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 115, 243, 50], OperandSize::Dword)
}

#[test]
fn psllq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM2)), operand2: Some(Literal8(75)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 115, 242, 75], OperandSize::Qword)
}

#[test]
fn psllq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM3)), operand2: Some(Literal8(115)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 115, 243, 115], OperandSize::Dword)
}

#[test]
fn psllq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM0)), operand2: Some(Literal8(127)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 115, 240, 127], OperandSize::Qword)
}

#[test]
fn psllq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 243, 205], OperandSize::Dword)
}

#[test]
fn psllq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 146180281, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 243, 28, 93, 185, 136, 182, 8], OperandSize::Dword)
}

#[test]
fn psllq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 243, 250], OperandSize::Qword)
}

#[test]
fn psllq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM0)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 243, 1], OperandSize::Qword)
}

#[test]
fn psllq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 243, 235], OperandSize::Dword)
}

#[test]
fn psllq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 1531771559, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 243, 188, 222, 167, 250, 76, 91], OperandSize::Dword)
}

#[test]
fn psllq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 243, 211], OperandSize::Qword)
}

#[test]
fn psllq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 243, 52, 192], OperandSize::Qword)
}

