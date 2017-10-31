use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn andpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 84, 255], OperandSize::Dword)
}

#[test]
fn andpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 84, 12, 115], OperandSize::Dword)
}

#[test]
fn andpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 84, 196], OperandSize::Qword)
}

#[test]
fn andpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RBX, 950857234, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 84, 163, 18, 238, 172, 56], OperandSize::Qword)
}

