use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubusw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 217, 205], OperandSize::Dword)
}

#[test]
fn psubusw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1933941973, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 217, 60, 77, 213, 156, 69, 115], OperandSize::Dword)
}

#[test]
fn psubusw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 217, 246], OperandSize::Qword)
}

#[test]
fn psubusw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Four, 493074620, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 217, 156, 135, 188, 184, 99, 29], OperandSize::Qword)
}

#[test]
fn psubusw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 217, 234], OperandSize::Dword)
}

#[test]
fn psubusw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1222319854, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 217, 28, 77, 238, 30, 219, 72], OperandSize::Dword)
}

#[test]
fn psubusw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 217, 200], OperandSize::Qword)
}

#[test]
fn psubusw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RDX, 2080455417, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 217, 138, 249, 58, 1, 124], OperandSize::Qword)
}

