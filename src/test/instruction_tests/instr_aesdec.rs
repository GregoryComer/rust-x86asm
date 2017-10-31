use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn aesdec_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDEC, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 222, 199], OperandSize::Dword)
}

#[test]
fn aesdec_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDEC, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EAX, 324866805, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 222, 176, 245, 18, 93, 19], OperandSize::Dword)
}

#[test]
fn aesdec_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDEC, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 222, 224], OperandSize::Qword)
}

#[test]
fn aesdec_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDEC, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RAX, 1628804107, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 222, 144, 11, 148, 21, 97], OperandSize::Qword)
}

