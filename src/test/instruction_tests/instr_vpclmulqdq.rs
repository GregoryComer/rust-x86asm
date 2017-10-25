use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpclmulqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCLMULQDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(79)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 68, 192, 79], OperandSize::Dword)
}

#[test]
fn vpclmulqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCLMULQDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Eight, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(10)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 68, 20, 249, 10], OperandSize::Dword)
}

#[test]
fn vpclmulqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCLMULQDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(36)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 68, 195, 36], OperandSize::Qword)
}

#[test]
fn vpclmulqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCLMULQDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 2041725155, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(101)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 68, 148, 88, 227, 64, 178, 121, 101], OperandSize::Qword)
}

