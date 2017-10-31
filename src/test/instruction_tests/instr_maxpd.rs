use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn maxpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 95, 194], OperandSize::Dword)
}

#[test]
fn maxpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EDI, EAX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 95, 4, 199], OperandSize::Dword)
}

#[test]
fn maxpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 95, 204], OperandSize::Qword)
}

#[test]
fn maxpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPD, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 95, 54], OperandSize::Qword)
}

