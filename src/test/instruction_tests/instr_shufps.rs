use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shufps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 198, 251, 82], OperandSize::Dword)
}

#[test]
fn shufps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 909533252, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 198, 188, 243, 68, 96, 54, 54, 46], OperandSize::Dword)
}

#[test]
fn shufps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 198, 241, 66], OperandSize::Qword)
}

#[test]
fn shufps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPS, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 198, 51, 6], OperandSize::Qword)
}

