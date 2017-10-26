use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpckhqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHQDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 109, 212], OperandSize::Dword)
}

#[test]
fn punpckhqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHQDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 2046896644, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 109, 60, 93, 4, 42, 1, 122], OperandSize::Dword)
}

#[test]
fn punpckhqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHQDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 109, 193], OperandSize::Qword)
}

#[test]
fn punpckhqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHQDQ, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 109, 38], OperandSize::Qword)
}

