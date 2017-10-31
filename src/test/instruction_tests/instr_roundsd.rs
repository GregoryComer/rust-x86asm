use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn roundsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 11, 219, 88], OperandSize::Dword)
}

#[test]
fn roundsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 1045345901, Some(OperandSize::Qword), None)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 11, 180, 130, 109, 182, 78, 62, 94], OperandSize::Dword)
}

#[test]
fn roundsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(25)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 11, 213, 25], OperandSize::Qword)
}

#[test]
fn roundsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Qword), None)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 11, 60, 73, 29], OperandSize::Qword)
}

