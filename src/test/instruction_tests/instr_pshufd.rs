use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pshufd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 112, 218, 35], OperandSize::Dword)
}

#[test]
fn pshufd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Four, 1496322387, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(17)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 112, 140, 159, 83, 17, 48, 89, 17], OperandSize::Dword)
}

#[test]
fn pshufd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(63)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 112, 253, 63], OperandSize::Qword)
}

#[test]
fn pshufd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1741107793, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 112, 60, 117, 81, 50, 199, 103, 32], OperandSize::Qword)
}

