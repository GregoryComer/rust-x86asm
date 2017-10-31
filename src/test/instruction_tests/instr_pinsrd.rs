use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pinsrd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(ESP)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 34, 196, 118], OperandSize::Dword)
}

#[test]
fn pinsrd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 1877318177, Some(OperandSize::Dword), None)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 34, 140, 191, 33, 154, 229, 111, 21], OperandSize::Dword)
}

#[test]
fn pinsrd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(EDX)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 34, 234, 99], OperandSize::Qword)
}

#[test]
fn pinsrd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRD, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 34, 33, 19], OperandSize::Qword)
}

