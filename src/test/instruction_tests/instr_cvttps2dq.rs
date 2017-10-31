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
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2DQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Eight, 624026730, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 91, 156, 219, 106, 228, 49, 37], OperandSize::Dword)
}

#[test]
fn cvttps2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2DQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 91, 254], OperandSize::Qword)
}

#[test]
fn cvttps2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2DQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 875472547, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 91, 52, 197, 163, 166, 46, 52], OperandSize::Qword)
}

