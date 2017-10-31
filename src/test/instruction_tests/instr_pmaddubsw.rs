use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaddubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 4, 206], OperandSize::Dword)
}

#[test]
fn pmaddubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(MM7)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 4, 57], OperandSize::Dword)
}

#[test]
fn pmaddubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 4, 201], OperandSize::Qword)
}

#[test]
fn pmaddubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 75899819, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 4, 180, 183, 171, 35, 134, 4], OperandSize::Qword)
}

#[test]
fn pmaddubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 4, 234], OperandSize::Dword)
}

#[test]
fn pmaddubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 1228265270, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 4, 4, 141, 54, 215, 53, 73], OperandSize::Dword)
}

#[test]
fn pmaddubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 4, 229], OperandSize::Qword)
}

#[test]
fn pmaddubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 4, 52, 72], OperandSize::Qword)
}

