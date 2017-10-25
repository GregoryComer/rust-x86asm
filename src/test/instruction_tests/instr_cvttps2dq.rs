use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvttps2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 91, 194], OperandSize::Dword)
}

#[test]
fn cvttps2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2DQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 91, 52, 90], OperandSize::Dword)
}

#[test]
fn cvttps2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2DQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 91, 210], OperandSize::Qword)
}

#[test]
fn cvttps2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2DQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RCX, 1442448752, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 91, 185, 112, 5, 250, 85], OperandSize::Qword)
}

