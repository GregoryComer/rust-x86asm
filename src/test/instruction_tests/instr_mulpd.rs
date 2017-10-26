use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn mulpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 89, 237], OperandSize::Dword)
}

#[test]
fn mulpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 89, 20, 88], OperandSize::Dword)
}

#[test]
fn mulpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 89, 244], OperandSize::Qword)
}

#[test]
fn mulpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPD, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 89, 33], OperandSize::Qword)
}

