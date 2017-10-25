use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcomiss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 47, 208], OperandSize::Dword)
}

#[test]
fn vcomiss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 47, 31], OperandSize::Dword)
}

#[test]
fn vcomiss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 47, 242], OperandSize::Qword)
}

#[test]
fn vcomiss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RAX, 1827127497, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 47, 176, 201, 192, 231, 108], OperandSize::Qword)
}

#[test]
fn vcomiss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 124, 24, 47, 199], OperandSize::Dword)
}

#[test]
fn vcomiss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Four, 1660102254, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 47, 148, 183, 110, 38, 243, 98], OperandSize::Dword)
}

#[test]
fn vcomiss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 124, 24, 47, 202], OperandSize::Qword)
}

#[test]
fn vcomiss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RDI, 848402367, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 47, 143, 191, 151, 145, 50], OperandSize::Qword)
}

