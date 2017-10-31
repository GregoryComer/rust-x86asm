use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaddwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 245, 203], OperandSize::Dword)
}

#[test]
fn pmaddwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 2062704706, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 245, 148, 243, 66, 96, 242, 122], OperandSize::Dword)
}

#[test]
fn pmaddwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 245, 246], OperandSize::Qword)
}

#[test]
fn pmaddwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(MM2)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 245, 19], OperandSize::Qword)
}

#[test]
fn pmaddwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 245, 245], OperandSize::Dword)
}

#[test]
fn pmaddwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 1777947429, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 245, 132, 119, 37, 83, 249, 105], OperandSize::Dword)
}

#[test]
fn pmaddwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 245, 226], OperandSize::Qword)
}

#[test]
fn pmaddwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RSI, 1605062428, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 245, 158, 28, 79, 171, 95], OperandSize::Qword)
}

