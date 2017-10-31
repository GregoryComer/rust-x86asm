use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pxor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 239, 205], OperandSize::Dword)
}

#[test]
fn pxor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(MM5)), operand2: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 239, 42], OperandSize::Dword)
}

#[test]
fn pxor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 239, 202], OperandSize::Qword)
}

#[test]
fn pxor_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(MM1)), operand2: Some(IndirectDisplaced(RCX, 1495447968, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 239, 137, 160, 185, 34, 89], OperandSize::Qword)
}

#[test]
fn pxor_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 239, 247], OperandSize::Dword)
}

#[test]
fn pxor_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 239, 44, 184], OperandSize::Dword)
}

#[test]
fn pxor_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 239, 211], OperandSize::Qword)
}

#[test]
fn pxor_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 239, 33], OperandSize::Qword)
}

