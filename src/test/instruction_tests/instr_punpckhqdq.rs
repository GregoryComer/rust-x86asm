use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpckhqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHQDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 109, 193], OperandSize::Dword)
}

#[test]
fn punpckhqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHQDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Eight, 1068308841, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 109, 188, 211, 105, 25, 173, 63], OperandSize::Dword)
}

#[test]
fn punpckhqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHQDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 109, 244], OperandSize::Qword)
}

#[test]
fn punpckhqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHQDQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 1796009746, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 109, 28, 77, 18, 239, 12, 107], OperandSize::Qword)
}

