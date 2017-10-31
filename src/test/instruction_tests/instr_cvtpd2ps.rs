use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtpd2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 90, 245], OperandSize::Dword)
}

#[test]
fn cvtpd2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(ESI, 974452008, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 90, 158, 40, 245, 20, 58], OperandSize::Dword)
}

#[test]
fn cvtpd2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 90, 195], OperandSize::Qword)
}

#[test]
fn cvtpd2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PS, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 90, 14], OperandSize::Qword)
}

