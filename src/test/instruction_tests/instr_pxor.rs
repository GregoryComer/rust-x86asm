use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pxor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 239, 245], OperandSize::Dword)
}

#[test]
fn pxor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 239, 28, 190], OperandSize::Dword)
}

#[test]
fn pxor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 239, 245], OperandSize::Qword)
}

#[test]
fn pxor_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 1873090299, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 239, 188, 80, 251, 22, 165, 111], OperandSize::Qword)
}

#[test]
fn pxor_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 239, 211], OperandSize::Dword)
}

#[test]
fn pxor_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 239, 44, 214], OperandSize::Dword)
}

#[test]
fn pxor_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 239, 238], OperandSize::Qword)
}

#[test]
fn pxor_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PXOR, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RDI, 524252866, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 239, 159, 194, 118, 63, 31], OperandSize::Qword)
}

