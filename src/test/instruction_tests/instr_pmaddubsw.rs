use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaddubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 4, 220], OperandSize::Dword)
}

#[test]
fn pmaddubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1617351576, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 4, 28, 181, 152, 211, 102, 96], OperandSize::Dword)
}

#[test]
fn pmaddubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 4, 203], OperandSize::Qword)
}

#[test]
fn pmaddubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 408357884, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 4, 140, 206, 252, 11, 87, 24], OperandSize::Qword)
}

#[test]
fn pmaddubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 4, 244], OperandSize::Dword)
}

#[test]
fn pmaddubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(EAX, 1778147510, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 4, 144, 182, 96, 252, 105], OperandSize::Dword)
}

#[test]
fn pmaddubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 4, 230], OperandSize::Qword)
}

#[test]
fn pmaddubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 4, 60, 242], OperandSize::Qword)
}

