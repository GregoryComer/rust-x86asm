use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 250, 231], OperandSize::Dword)
}

#[test]
fn psubd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(MM5)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 250, 41], OperandSize::Dword)
}

#[test]
fn psubd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 250, 234], OperandSize::Qword)
}

#[test]
fn psubd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(MM5)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 250, 40], OperandSize::Qword)
}

#[test]
fn psubd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 250, 211], OperandSize::Dword)
}

#[test]
fn psubd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Eight, 1501264372, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 250, 172, 216, 244, 121, 123, 89], OperandSize::Dword)
}

#[test]
fn psubd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 250, 247], OperandSize::Qword)
}

#[test]
fn psubd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1234669762, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 250, 36, 69, 194, 144, 151, 73], OperandSize::Qword)
}

