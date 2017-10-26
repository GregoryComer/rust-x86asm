use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psignb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 8, 237], OperandSize::Dword)
}

#[test]
fn psignb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(MM1)), operand2: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 8, 15], OperandSize::Dword)
}

#[test]
fn psignb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 8, 206], OperandSize::Qword)
}

#[test]
fn psignb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 746022036, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 8, 60, 77, 148, 100, 119, 44], OperandSize::Qword)
}

#[test]
fn psignb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 8, 220], OperandSize::Dword)
}

#[test]
fn psignb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EDI, 1194588982, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 8, 183, 54, 251, 51, 71], OperandSize::Dword)
}

#[test]
fn psignb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 8, 244], OperandSize::Qword)
}

#[test]
fn psignb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Two, 1907650737, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 8, 188, 122, 177, 112, 180, 113], OperandSize::Qword)
}

