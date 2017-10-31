use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpclmulqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCLMULQDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(35)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 68, 193, 35], OperandSize::Dword)
}

#[test]
fn vpclmulqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCLMULQDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1821125344, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(119)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 68, 52, 189, 224, 42, 140, 108, 119], OperandSize::Dword)
}

#[test]
fn vpclmulqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCLMULQDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(103)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 68, 207, 103], OperandSize::Qword)
}

#[test]
fn vpclmulqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCLMULQDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 68, 4, 183, 89], OperandSize::Qword)
}

