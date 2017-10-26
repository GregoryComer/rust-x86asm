use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtpd2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 90, 194], OperandSize::Dword)
}

#[test]
fn cvtpd2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 691765646, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 90, 28, 221, 142, 129, 59, 41], OperandSize::Dword)
}

#[test]
fn cvtpd2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 90, 246], OperandSize::Qword)
}

#[test]
fn cvtpd2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RDI, 833951990, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 90, 151, 246, 24, 181, 49], OperandSize::Qword)
}

