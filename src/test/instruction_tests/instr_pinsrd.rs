use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pinsrd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(EDX)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 34, 250, 9], OperandSize::Dword)
}

#[test]
fn pinsrd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(ESI, 1950643881, Some(OperandSize::Dword), None)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 34, 166, 169, 118, 68, 116, 95], OperandSize::Dword)
}

#[test]
fn pinsrd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(EDX)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 34, 210, 26], OperandSize::Qword)
}

#[test]
fn pinsrd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Dword), None)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 34, 12, 159, 73], OperandSize::Qword)
}

