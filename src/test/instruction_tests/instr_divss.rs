use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn divss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 94, 214], OperandSize::Dword)
}

#[test]
fn divss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(EAX, 814994884, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 94, 152, 196, 213, 147, 48], OperandSize::Dword)
}

#[test]
fn divss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 94, 207], OperandSize::Qword)
}

#[test]
fn divss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSS, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 94, 23], OperandSize::Qword)
}

