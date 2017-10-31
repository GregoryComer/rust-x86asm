use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtps2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 91, 196], OperandSize::Dword)
}

#[test]
fn cvtps2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2DQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EBX, 1336661202, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 91, 131, 210, 212, 171, 79], OperandSize::Dword)
}

#[test]
fn cvtps2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2DQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 91, 206], OperandSize::Qword)
}

#[test]
fn cvtps2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2DQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 91, 12, 246], OperandSize::Qword)
}

