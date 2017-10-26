use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psllw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM6)), operand2: Some(Literal8(27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 246, 27], OperandSize::Dword)
}

#[test]
fn psllw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM1)), operand2: Some(Literal8(70)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 241, 70], OperandSize::Qword)
}

#[test]
fn psllw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM3)), operand2: Some(Literal8(28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 243, 28], OperandSize::Dword)
}

#[test]
fn psllw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM7)), operand2: Some(Literal8(91)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 247, 91], OperandSize::Qword)
}

#[test]
fn psllw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 241, 202], OperandSize::Dword)
}

#[test]
fn psllw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 1771598978, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 241, 140, 67, 130, 116, 152, 105], OperandSize::Dword)
}

#[test]
fn psllw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 241, 201], OperandSize::Qword)
}

#[test]
fn psllw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(RBX, 1475371985, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 241, 147, 209, 99, 240, 87], OperandSize::Qword)
}

#[test]
fn psllw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 241, 232], OperandSize::Dword)
}

#[test]
fn psllw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1515003016, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 241, 4, 69, 136, 28, 77, 90], OperandSize::Dword)
}

#[test]
fn psllw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 241, 237], OperandSize::Qword)
}

#[test]
fn psllw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 241, 55], OperandSize::Qword)
}

