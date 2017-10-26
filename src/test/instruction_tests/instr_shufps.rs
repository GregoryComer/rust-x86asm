use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shufps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 198, 196, 126], OperandSize::Dword)
}

#[test]
fn shufps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPS, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 198, 58, 33], OperandSize::Dword)
}

#[test]
fn shufps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(105)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 198, 250, 105], OperandSize::Qword)
}

#[test]
fn shufps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 198, 28, 208, 104], OperandSize::Qword)
}

