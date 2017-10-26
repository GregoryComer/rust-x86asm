use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn unpckhps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 21, 214], OperandSize::Dword)
}

#[test]
fn unpckhps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 21, 4, 246], OperandSize::Dword)
}

#[test]
fn unpckhps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 21, 219], OperandSize::Qword)
}

#[test]
fn unpckhps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 957326199, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 21, 20, 197, 119, 163, 15, 57], OperandSize::Qword)
}

