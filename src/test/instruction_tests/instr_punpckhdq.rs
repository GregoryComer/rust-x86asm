use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpckhdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 106, 192], OperandSize::Dword)
}

#[test]
fn punpckhdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(MM2)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 106, 16], OperandSize::Dword)
}

#[test]
fn punpckhdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 106, 217], OperandSize::Qword)
}

#[test]
fn punpckhdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 392161863, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 106, 172, 152, 71, 234, 95, 23], OperandSize::Qword)
}

#[test]
fn punpckhdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 106, 212], OperandSize::Dword)
}

#[test]
fn punpckhdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 106, 43], OperandSize::Dword)
}

#[test]
fn punpckhdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 106, 216], OperandSize::Qword)
}

#[test]
fn punpckhdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RSI, 1225794183, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 106, 174, 135, 34, 16, 73], OperandSize::Qword)
}

