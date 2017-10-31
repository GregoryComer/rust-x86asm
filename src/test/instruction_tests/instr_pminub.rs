use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pminub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 218, 250], OperandSize::Dword)
}

#[test]
fn pminub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(MM3)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 218, 30], OperandSize::Dword)
}

#[test]
fn pminub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 218, 202], OperandSize::Qword)
}

#[test]
fn pminub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 218, 60, 202], OperandSize::Qword)
}

#[test]
fn pminub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 218, 209], OperandSize::Dword)
}

#[test]
fn pminub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 269306470, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 218, 180, 87, 102, 74, 13, 16], OperandSize::Dword)
}

#[test]
fn pminub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 218, 232], OperandSize::Qword)
}

#[test]
fn pminub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 717013684, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 218, 20, 253, 180, 194, 188, 42], OperandSize::Qword)
}

