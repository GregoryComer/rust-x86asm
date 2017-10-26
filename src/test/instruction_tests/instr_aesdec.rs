use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn aesdec_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDEC, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 222, 248], OperandSize::Dword)
}

#[test]
fn aesdec_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDEC, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(ESI, EBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 222, 28, 94], OperandSize::Dword)
}

#[test]
fn aesdec_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDEC, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 222, 255], OperandSize::Qword)
}

#[test]
fn aesdec_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDEC, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 222, 11], OperandSize::Qword)
}

