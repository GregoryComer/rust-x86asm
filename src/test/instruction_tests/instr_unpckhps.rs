use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn unpckhps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 21, 224], OperandSize::Dword)
}

#[test]
fn unpckhps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EBX, EDI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 21, 44, 251], OperandSize::Dword)
}

#[test]
fn unpckhps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 21, 231], OperandSize::Qword)
}

#[test]
fn unpckhps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 1797673412, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 21, 36, 181, 196, 81, 38, 107], OperandSize::Qword)
}

