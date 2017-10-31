use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psignw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 9, 220], OperandSize::Dword)
}

#[test]
fn psignw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(MM1)), operand2: Some(IndirectDisplaced(EDI, 1871959324, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 9, 143, 28, 213, 147, 111], OperandSize::Dword)
}

#[test]
fn psignw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 9, 245], OperandSize::Qword)
}

#[test]
fn psignw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Two, 1356983846, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 9, 172, 112, 38, 238, 225, 80], OperandSize::Qword)
}

#[test]
fn psignw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 9, 233], OperandSize::Dword)
}

#[test]
fn psignw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 9, 0], OperandSize::Dword)
}

#[test]
fn psignw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 9, 253], OperandSize::Qword)
}

#[test]
fn psignw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Four, 2106654515, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 9, 180, 159, 51, 255, 144, 125], OperandSize::Qword)
}

