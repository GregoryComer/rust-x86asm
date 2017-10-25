use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaxsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 60, 249], OperandSize::Dword)
}

#[test]
fn pmaxsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSB, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Two, 617212687, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 60, 164, 74, 15, 235, 201, 36], OperandSize::Dword)
}

#[test]
fn pmaxsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 60, 212], OperandSize::Qword)
}

#[test]
fn pmaxsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSB, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 620599623, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 60, 148, 190, 71, 153, 253, 36], OperandSize::Qword)
}

