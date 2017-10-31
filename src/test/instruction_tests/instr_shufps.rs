use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shufps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 198, 198, 9], OperandSize::Dword)
}

#[test]
fn shufps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(ESI, 135520829, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 198, 150, 61, 226, 19, 8, 118], OperandSize::Dword)
}

#[test]
fn shufps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 198, 248, 74], OperandSize::Qword)
}

#[test]
fn shufps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 145422894, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 198, 60, 245, 46, 250, 170, 8, 16], OperandSize::Qword)
}

