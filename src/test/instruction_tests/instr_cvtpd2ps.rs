use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtpd2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 90, 203], OperandSize::Dword)
}

#[test]
fn cvtpd2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 940279082, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 90, 20, 125, 42, 133, 11, 56], OperandSize::Dword)
}

#[test]
fn cvtpd2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 90, 249], OperandSize::Qword)
}

#[test]
fn cvtpd2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 90, 60, 219], OperandSize::Qword)
}

