use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovsxwq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 36, 250], OperandSize::Dword)
}

#[test]
fn pmovsxwq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 850212671, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 36, 60, 197, 63, 55, 173, 50], OperandSize::Dword)
}

#[test]
fn pmovsxwq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 36, 200], OperandSize::Qword)
}

#[test]
fn pmovsxwq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 440907012, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 36, 4, 133, 4, 181, 71, 26], OperandSize::Qword)
}

