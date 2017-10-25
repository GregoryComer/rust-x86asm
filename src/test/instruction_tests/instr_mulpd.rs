use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn mulpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 89, 202], OperandSize::Dword)
}

#[test]
fn mulpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(EDI, 2042172413, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 89, 151, 253, 19, 185, 121], OperandSize::Dword)
}

#[test]
fn mulpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 89, 236], OperandSize::Qword)
}

#[test]
fn mulpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1176991446, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 89, 28, 69, 214, 118, 39, 70], OperandSize::Qword)
}

