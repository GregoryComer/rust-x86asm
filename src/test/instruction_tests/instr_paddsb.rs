use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 236, 222], OperandSize::Dword)
}

#[test]
fn paddsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 442206168, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 236, 156, 178, 216, 135, 91, 26], OperandSize::Dword)
}

#[test]
fn paddsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 236, 192], OperandSize::Qword)
}

#[test]
fn paddsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 26619622, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 236, 20, 253, 230, 46, 150, 1], OperandSize::Qword)
}

#[test]
fn paddsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 236, 202], OperandSize::Dword)
}

#[test]
fn paddsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(ECX, 548114239, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 236, 153, 63, 143, 171, 32], OperandSize::Dword)
}

#[test]
fn paddsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 236, 241], OperandSize::Qword)
}

#[test]
fn paddsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1677048819, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 236, 52, 93, 243, 187, 245, 99], OperandSize::Qword)
}

