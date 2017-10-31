use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 248, 199], OperandSize::Dword)
}

#[test]
fn psubb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexed(EDX, ECX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 248, 20, 138], OperandSize::Dword)
}

#[test]
fn psubb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 248, 240], OperandSize::Qword)
}

#[test]
fn psubb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(RAX, 1788002700, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 248, 184, 140, 193, 146, 106], OperandSize::Qword)
}

#[test]
fn psubb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 248, 242], OperandSize::Dword)
}

#[test]
fn psubb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EBX, 1637571860, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 248, 131, 20, 93, 155, 97], OperandSize::Dword)
}

#[test]
fn psubb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 248, 216], OperandSize::Qword)
}

#[test]
fn psubb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1577563526, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 248, 20, 141, 134, 181, 7, 94], OperandSize::Qword)
}

