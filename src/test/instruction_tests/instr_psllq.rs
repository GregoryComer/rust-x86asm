use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psllq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM4)), operand2: Some(Literal8(72)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 115, 244, 72], OperandSize::Dword)
}

#[test]
fn psllq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM5)), operand2: Some(Literal8(30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 115, 245, 30], OperandSize::Qword)
}

#[test]
fn psllq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM3)), operand2: Some(Literal8(96)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 115, 243, 96], OperandSize::Dword)
}

#[test]
fn psllq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM7)), operand2: Some(Literal8(105)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 115, 247, 105], OperandSize::Qword)
}

#[test]
fn psllq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 243, 249], OperandSize::Dword)
}

#[test]
fn psllq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM6)), operand2: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 243, 51], OperandSize::Dword)
}

#[test]
fn psllq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 243, 251], OperandSize::Qword)
}

#[test]
fn psllq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 243, 12, 248], OperandSize::Qword)
}

#[test]
fn psllq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 243, 195], OperandSize::Dword)
}

#[test]
fn psllq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Four, 1454917454, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 243, 164, 137, 78, 71, 184, 86], OperandSize::Dword)
}

#[test]
fn psllq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 243, 197], OperandSize::Qword)
}

#[test]
fn psllq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RDX, 199383206, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 243, 162, 166, 88, 226, 11], OperandSize::Qword)
}

