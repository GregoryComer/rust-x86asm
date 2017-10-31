use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaddwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 245, 252], OperandSize::Dword)
}

#[test]
fn pmaddwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexed(EDI, ESI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 245, 52, 119], OperandSize::Dword)
}

#[test]
fn pmaddwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 245, 208], OperandSize::Qword)
}

#[test]
fn pmaddwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 958624076, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 245, 4, 205, 76, 113, 35, 57], OperandSize::Qword)
}

#[test]
fn pmaddwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 245, 250], OperandSize::Dword)
}

#[test]
fn pmaddwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 249307183, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 245, 4, 149, 47, 32, 220, 14], OperandSize::Dword)
}

#[test]
fn pmaddwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 245, 217], OperandSize::Qword)
}

#[test]
fn pmaddwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 1004492258, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 245, 36, 221, 226, 85, 223, 59], OperandSize::Qword)
}

