use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpckhqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHQDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 109, 218], OperandSize::Dword)
}

#[test]
fn punpckhqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHQDQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 109, 44, 73], OperandSize::Dword)
}

#[test]
fn punpckhqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHQDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 109, 243], OperandSize::Qword)
}

#[test]
fn punpckhqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHQDQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 825153381, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 109, 164, 113, 101, 215, 46, 49], OperandSize::Qword)
}

