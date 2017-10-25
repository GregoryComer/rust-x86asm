use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtpd2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2DQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 230, 221], OperandSize::Dword)
}

#[test]
fn cvtpd2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2DQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EBX, 1221678369, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 230, 131, 33, 85, 209, 72], OperandSize::Dword)
}

#[test]
fn cvtpd2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2DQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 230, 211], OperandSize::Qword)
}

#[test]
fn cvtpd2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2DQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Eight, 1962255546, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 230, 132, 214, 186, 164, 245, 116], OperandSize::Qword)
}

