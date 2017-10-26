use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pclmulqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCLMULQDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(17)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 68, 242, 17], OperandSize::Dword)
}

#[test]
fn pclmulqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCLMULQDQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(28)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 68, 12, 147, 28], OperandSize::Dword)
}

#[test]
fn pclmulqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCLMULQDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 68, 240, 104], OperandSize::Qword)
}

#[test]
fn pclmulqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCLMULQDQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 248911581, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 68, 36, 85, 221, 22, 214, 14, 7], OperandSize::Qword)
}

