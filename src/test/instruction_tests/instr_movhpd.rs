use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movhpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 22, 24], OperandSize::Dword)
}

#[test]
fn movhpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 22, 4, 187], OperandSize::Qword)
}

#[test]
fn movhpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPD, operand1: Some(IndirectDisplaced(EDX, 675491729, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 23, 146, 145, 47, 67, 40], OperandSize::Dword)
}

#[test]
fn movhpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPD, operand1: Some(IndirectDisplaced(RBX, 1645203596, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 23, 155, 140, 208, 15, 98], OperandSize::Qword)
}

