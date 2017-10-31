use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpistri_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRI, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 99, 250, 15], OperandSize::Dword)
}

#[test]
fn pcmpistri_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRI, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EBX, 1175881343, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 99, 163, 127, 134, 22, 70, 122], OperandSize::Dword)
}

#[test]
fn pcmpistri_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRI, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 99, 207, 86], OperandSize::Qword)
}

#[test]
fn pcmpistri_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRI, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 99, 49, 51], OperandSize::Qword)
}

