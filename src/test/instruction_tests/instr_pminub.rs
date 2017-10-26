use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pminub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 218, 251], OperandSize::Dword)
}

#[test]
fn pminub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1813617489, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 218, 12, 125, 81, 155, 25, 108], OperandSize::Dword)
}

#[test]
fn pminub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 218, 248], OperandSize::Qword)
}

#[test]
fn pminub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(MM7)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 218, 59], OperandSize::Qword)
}

#[test]
fn pminub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 218, 229], OperandSize::Dword)
}

#[test]
fn pminub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 218, 25], OperandSize::Dword)
}

#[test]
fn pminub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 218, 194], OperandSize::Qword)
}

#[test]
fn pminub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1479556578, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 218, 44, 245, 226, 61, 48, 88], OperandSize::Qword)
}

