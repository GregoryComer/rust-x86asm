use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn maxpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 95, 193], OperandSize::Dword)
}

#[test]
fn maxpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 95, 28, 200], OperandSize::Dword)
}

#[test]
fn maxpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 95, 199], OperandSize::Qword)
}

#[test]
fn maxpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RDX, 2032989563, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 95, 138, 123, 245, 44, 121], OperandSize::Qword)
}

