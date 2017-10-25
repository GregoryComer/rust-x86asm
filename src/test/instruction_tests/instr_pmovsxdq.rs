use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovsxdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 37, 208], OperandSize::Dword)
}

#[test]
fn pmovsxdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXDQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 37, 20, 75], OperandSize::Dword)
}

#[test]
fn pmovsxdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 37, 196], OperandSize::Qword)
}

#[test]
fn pmovsxdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXDQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RCX, 316754172, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 37, 137, 252, 72, 225, 18], OperandSize::Qword)
}

