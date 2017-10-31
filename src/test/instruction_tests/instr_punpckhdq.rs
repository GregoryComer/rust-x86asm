use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpckhdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 106, 220], OperandSize::Dword)
}

#[test]
fn punpckhdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(MM0)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 106, 6], OperandSize::Dword)
}

#[test]
fn punpckhdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 106, 248], OperandSize::Qword)
}

#[test]
fn punpckhdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(RDI, 1918108774, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 106, 191, 102, 4, 84, 114], OperandSize::Qword)
}

#[test]
fn punpckhdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 106, 215], OperandSize::Dword)
}

#[test]
fn punpckhdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Two, 1983342756, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 106, 132, 66, 164, 104, 55, 118], OperandSize::Dword)
}

#[test]
fn punpckhdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 106, 203], OperandSize::Qword)
}

#[test]
fn punpckhdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 162130635, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 106, 36, 77, 203, 234, 169, 9], OperandSize::Qword)
}

