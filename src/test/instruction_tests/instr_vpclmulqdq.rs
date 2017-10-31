use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpclmulqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCLMULQDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(27)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 68, 198, 27], OperandSize::Dword)
}

#[test]
fn vpclmulqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCLMULQDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1501826694, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(11)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 68, 52, 125, 134, 14, 132, 89, 11], OperandSize::Dword)
}

#[test]
fn vpclmulqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCLMULQDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(23)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 68, 209, 23], OperandSize::Qword)
}

#[test]
fn vpclmulqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCLMULQDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 1750665894, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(30)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 68, 140, 80, 166, 10, 89, 104, 30], OperandSize::Qword)
}

