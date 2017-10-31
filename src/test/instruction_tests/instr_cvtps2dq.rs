use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtps2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2DQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 91, 208], OperandSize::Dword)
}

#[test]
fn cvtps2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 91, 0], OperandSize::Dword)
}

#[test]
fn cvtps2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2DQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 91, 219], OperandSize::Qword)
}

#[test]
fn cvtps2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2DQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Eight, 1190752689, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 91, 188, 217, 177, 113, 249, 70], OperandSize::Qword)
}

