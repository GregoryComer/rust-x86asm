use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtpd2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2DQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 230, 212], OperandSize::Dword)
}

#[test]
fn cvtpd2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2DQ, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 230, 15], OperandSize::Dword)
}

#[test]
fn cvtpd2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2DQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 230, 204], OperandSize::Qword)
}

#[test]
fn cvtpd2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2DQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 230, 4, 137], OperandSize::Qword)
}

