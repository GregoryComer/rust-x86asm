use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaddwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 245, 249], OperandSize::Dword)
}

#[test]
fn pmaddwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Eight, 927091909, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 245, 156, 250, 197, 76, 66, 55], OperandSize::Dword)
}

#[test]
fn pmaddwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 245, 205], OperandSize::Qword)
}

#[test]
fn pmaddwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 2103271722, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 245, 180, 152, 42, 97, 93, 125], OperandSize::Qword)
}

#[test]
fn pmaddwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 245, 241], OperandSize::Dword)
}

#[test]
fn pmaddwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Four, 625483036, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 245, 164, 139, 28, 29, 72, 37], OperandSize::Dword)
}

#[test]
fn pmaddwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 245, 196], OperandSize::Qword)
}

#[test]
fn pmaddwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RDI, 855410556, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 245, 143, 124, 135, 252, 50], OperandSize::Qword)
}

