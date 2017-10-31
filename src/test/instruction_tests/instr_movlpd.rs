use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movlpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(ESI, 341756989, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 18, 182, 61, 204, 94, 20], OperandSize::Dword)
}

#[test]
fn movlpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RDX, 1840242327, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 18, 186, 151, 222, 175, 109], OperandSize::Qword)
}

#[test]
fn movlpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPD, operand1: Some(IndirectScaledDisplaced(EDI, Four, 1531533360, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 19, 60, 189, 48, 88, 73, 91], OperandSize::Dword)
}

#[test]
fn movlpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPD, operand1: Some(IndirectScaledIndexed(RAX, RBX, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 19, 60, 152], OperandSize::Qword)
}

