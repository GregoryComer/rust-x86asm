use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pabsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 28, 213], OperandSize::Dword)
}

#[test]
fn pabsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(EDI, 19817390, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 28, 151, 174, 99, 46, 1], OperandSize::Dword)
}

#[test]
fn pabsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 28, 211], OperandSize::Qword)
}

#[test]
fn pabsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 1919520996, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 28, 188, 147, 228, 144, 105, 114], OperandSize::Qword)
}

#[test]
fn pabsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 28, 195], OperandSize::Dword)
}

#[test]
fn pabsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 912654662, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 28, 12, 93, 70, 1, 102, 54], OperandSize::Dword)
}

#[test]
fn pabsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 28, 205], OperandSize::Qword)
}

#[test]
fn pabsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 28, 59], OperandSize::Qword)
}

