use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpckhqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHQDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 109, 244], OperandSize::Dword)
}

#[test]
fn punpckhqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHQDQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EAX, 118747637, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 109, 160, 245, 241, 19, 7], OperandSize::Dword)
}

#[test]
fn punpckhqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHQDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 109, 220], OperandSize::Qword)
}

#[test]
fn punpckhqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHQDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 109, 60, 82], OperandSize::Qword)
}

