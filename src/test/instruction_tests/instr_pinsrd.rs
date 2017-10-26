use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pinsrd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(ESP)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 34, 252, 38], OperandSize::Dword)
}

#[test]
fn pinsrd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(63)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 34, 24, 63], OperandSize::Dword)
}

#[test]
fn pinsrd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(EBX)), operand3: Some(Literal8(37)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 34, 243, 37], OperandSize::Qword)
}

#[test]
fn pinsrd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRD, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 34, 54, 88], OperandSize::Qword)
}

