use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn phsubd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 6, 214], OperandSize::Dword)
}

#[test]
fn phsubd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1510807738, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 6, 4, 77, 186, 24, 13, 90], OperandSize::Dword)
}

#[test]
fn phsubd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 6, 232], OperandSize::Qword)
}

#[test]
fn phsubd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Four, 1543104577, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 6, 164, 184, 65, 232, 249, 91], OperandSize::Qword)
}

#[test]
fn phsubd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 6, 197], OperandSize::Dword)
}

#[test]
fn phsubd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 1039742092, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 6, 20, 117, 140, 52, 249, 61], OperandSize::Dword)
}

#[test]
fn phsubd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 6, 227], OperandSize::Qword)
}

#[test]
fn phsubd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 1828052283, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 6, 164, 192, 59, 221, 245, 108], OperandSize::Qword)
}

