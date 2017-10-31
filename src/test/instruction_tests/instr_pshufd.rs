use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pshufd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 112, 236, 119], OperandSize::Dword)
}

#[test]
fn pshufd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(103)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 112, 59, 103], OperandSize::Dword)
}

#[test]
fn pshufd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(30)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 112, 211, 30], OperandSize::Qword)
}

#[test]
fn pshufd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(17)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 112, 4, 206, 17], OperandSize::Qword)
}

