use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psllw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM6)), operand2: Some(Literal8(86)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 246, 86], OperandSize::Dword)
}

#[test]
fn psllw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM7)), operand2: Some(Literal8(114)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 247, 114], OperandSize::Qword)
}

#[test]
fn psllw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM4)), operand2: Some(Literal8(104)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 244, 104], OperandSize::Dword)
}

#[test]
fn psllw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM2)), operand2: Some(Literal8(40)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 242, 40], OperandSize::Qword)
}

#[test]
fn psllw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 241, 255], OperandSize::Dword)
}

#[test]
fn psllw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM4)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 241, 32], OperandSize::Dword)
}

#[test]
fn psllw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 241, 251], OperandSize::Qword)
}

#[test]
fn psllw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 300355329, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 241, 44, 213, 1, 15, 231, 17], OperandSize::Qword)
}

#[test]
fn psllw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 241, 210], OperandSize::Dword)
}

#[test]
fn psllw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 241, 17], OperandSize::Dword)
}

#[test]
fn psllw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 241, 211], OperandSize::Qword)
}

#[test]
fn psllw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 241, 30], OperandSize::Qword)
}

