use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovsxbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 32, 230], OperandSize::Dword)
}

#[test]
fn pmovsxbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 169519838, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 32, 28, 189, 222, 170, 26, 10], OperandSize::Dword)
}

#[test]
fn pmovsxbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 32, 252], OperandSize::Qword)
}

#[test]
fn pmovsxbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RDX, 175915544, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 32, 138, 24, 66, 124, 10], OperandSize::Qword)
}

