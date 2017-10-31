use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pclmulqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCLMULQDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 68, 245, 34], OperandSize::Dword)
}

#[test]
fn pclmulqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCLMULQDQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 68, 20, 88, 82], OperandSize::Dword)
}

#[test]
fn pclmulqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCLMULQDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 68, 194, 2], OperandSize::Qword)
}

#[test]
fn pclmulqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCLMULQDQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 486684225, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 68, 172, 187, 65, 54, 2, 29, 83], OperandSize::Qword)
}

