use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn addps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 88, 222], OperandSize::Dword)
}

#[test]
fn addps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(ECX, 932555068, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 88, 129, 60, 169, 149, 55], OperandSize::Dword)
}

#[test]
fn addps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 88, 226], OperandSize::Qword)
}

#[test]
fn addps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RAX, 199138286, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 88, 184, 238, 155, 222, 11], OperandSize::Qword)
}

