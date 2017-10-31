use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtpd2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2DQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 230, 218], OperandSize::Dword)
}

#[test]
fn cvtpd2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2DQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 1255963982, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 230, 52, 197, 78, 125, 220, 74], OperandSize::Dword)
}

#[test]
fn cvtpd2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2DQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 230, 245], OperandSize::Qword)
}

#[test]
fn cvtpd2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2DQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 1890108756, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 230, 36, 77, 84, 197, 168, 112], OperandSize::Qword)
}

