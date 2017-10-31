use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movhpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EBX, EBX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 22, 36, 155], OperandSize::Dword)
}

#[test]
fn movhpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RDI, 954220987, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 22, 183, 187, 65, 224, 56], OperandSize::Qword)
}

#[test]
fn movhpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPD, operand1: Some(IndirectScaledDisplaced(ESI, Eight, 1175168616, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 23, 52, 245, 104, 166, 11, 70], OperandSize::Dword)
}

#[test]
fn movhpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPD, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 2126469565, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 23, 44, 213, 189, 89, 191, 126], OperandSize::Qword)
}

