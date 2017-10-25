use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shufps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 198, 227, 92], OperandSize::Dword)
}

#[test]
fn shufps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 198, 28, 217, 80], OperandSize::Dword)
}

#[test]
fn shufps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 198, 251, 95], OperandSize::Qword)
}

#[test]
fn shufps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 650580179, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 198, 20, 77, 211, 16, 199, 38, 76], OperandSize::Qword)
}

