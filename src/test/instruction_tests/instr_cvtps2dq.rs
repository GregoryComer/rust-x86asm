use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtps2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 91, 197], OperandSize::Dword)
}

#[test]
fn cvtps2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2DQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EDX, 1963228769, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 91, 162, 97, 126, 4, 117], OperandSize::Dword)
}

#[test]
fn cvtps2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2DQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 91, 222], OperandSize::Qword)
}

#[test]
fn cvtps2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2DQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 308147726, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 91, 12, 245, 14, 246, 93, 18], OperandSize::Qword)
}

