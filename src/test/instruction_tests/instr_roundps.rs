use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn roundps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(78)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 8, 231, 78], OperandSize::Dword)
}

#[test]
fn roundps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 1579016968, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(105)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 8, 44, 197, 8, 227, 29, 94, 105], OperandSize::Dword)
}

#[test]
fn roundps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(100)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 8, 194, 100], OperandSize::Qword)
}

#[test]
fn roundps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPS, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 8, 30, 62], OperandSize::Qword)
}

