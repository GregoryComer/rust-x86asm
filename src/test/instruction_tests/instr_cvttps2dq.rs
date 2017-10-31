use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvttps2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2DQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 91, 247], OperandSize::Dword)
}

#[test]
fn cvttps2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2DQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Eight, 1134454754, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 91, 180, 211, 226, 103, 158, 67], OperandSize::Dword)
}

#[test]
fn cvttps2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2DQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 91, 213], OperandSize::Qword)
}

#[test]
fn cvttps2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2DQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 1205250639, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 91, 156, 74, 79, 170, 214, 71], OperandSize::Qword)
}

