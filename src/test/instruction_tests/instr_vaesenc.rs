use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaesenc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENC, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 220, 195], OperandSize::Dword)
}

#[test]
fn vaesenc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENC, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EBX, 1244765752, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 220, 187, 56, 158, 49, 74], OperandSize::Dword)
}

#[test]
fn vaesenc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENC, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 220, 215], OperandSize::Qword)
}

#[test]
fn vaesenc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENC, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RAX, 2056541136, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 220, 144, 208, 83, 148, 122], OperandSize::Qword)
}

