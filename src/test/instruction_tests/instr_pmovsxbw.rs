use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovsxbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 32, 229], OperandSize::Dword)
}

#[test]
fn pmovsxbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 32, 52, 152], OperandSize::Dword)
}

#[test]
fn pmovsxbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 32, 203], OperandSize::Qword)
}

#[test]
fn pmovsxbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 1194454817, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 32, 180, 200, 33, 239, 49, 71], OperandSize::Qword)
}

