use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpclmulqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCLMULQDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(105)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 68, 206, 105], OperandSize::Dword)
}

#[test]
fn vpclmulqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCLMULQDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(10)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 68, 11, 10], OperandSize::Dword)
}

#[test]
fn vpclmulqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCLMULQDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 68, 241, 108], OperandSize::Qword)
}

#[test]
fn vpclmulqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCLMULQDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(22)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 68, 60, 208, 22], OperandSize::Qword)
}

