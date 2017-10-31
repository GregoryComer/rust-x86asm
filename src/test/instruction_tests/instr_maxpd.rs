use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn maxpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 95, 200], OperandSize::Dword)
}

#[test]
fn maxpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1514795097, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 95, 44, 245, 89, 240, 73, 90], OperandSize::Dword)
}

#[test]
fn maxpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 95, 222], OperandSize::Qword)
}

#[test]
fn maxpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 822593841, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 95, 60, 213, 49, 201, 7, 49], OperandSize::Qword)
}

