use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 254, 216], OperandSize::Dword)
}

#[test]
fn paddd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 254, 36, 254], OperandSize::Dword)
}

#[test]
fn paddd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 254, 217], OperandSize::Qword)
}

#[test]
fn paddd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 1142121925, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 254, 28, 77, 197, 101, 19, 68], OperandSize::Qword)
}

#[test]
fn paddd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 254, 205], OperandSize::Dword)
}

#[test]
fn paddd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 1539639294, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 254, 20, 117, 254, 7, 197, 91], OperandSize::Dword)
}

#[test]
fn paddd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 254, 215], OperandSize::Qword)
}

#[test]
fn paddd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 138053807, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 254, 36, 69, 175, 136, 58, 8], OperandSize::Qword)
}

