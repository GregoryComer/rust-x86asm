use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 251, 213], OperandSize::Dword)
}

#[test]
fn psubq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 982921233, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 251, 20, 117, 17, 48, 150, 58], OperandSize::Dword)
}

#[test]
fn psubq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 251, 229], OperandSize::Qword)
}

#[test]
fn psubq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(RDX, 398723274, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 251, 146, 202, 8, 196, 23], OperandSize::Qword)
}

#[test]
fn psubq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 251, 223], OperandSize::Dword)
}

#[test]
fn psubq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1334290295, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 251, 36, 181, 119, 167, 135, 79], OperandSize::Dword)
}

#[test]
fn psubq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 251, 230], OperandSize::Qword)
}

#[test]
fn psubq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1234649104, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 251, 60, 117, 16, 64, 151, 73], OperandSize::Qword)
}

