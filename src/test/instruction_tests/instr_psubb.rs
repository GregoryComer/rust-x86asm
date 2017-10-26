use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 248, 253], OperandSize::Dword)
}

#[test]
fn psubb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexed(EDI, EBX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 248, 12, 95], OperandSize::Dword)
}

#[test]
fn psubb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 248, 234], OperandSize::Qword)
}

#[test]
fn psubb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1368436896, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 248, 20, 69, 160, 176, 144, 81], OperandSize::Qword)
}

#[test]
fn psubb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 248, 253], OperandSize::Dword)
}

#[test]
fn psubb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 248, 46], OperandSize::Dword)
}

#[test]
fn psubb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 248, 193], OperandSize::Qword)
}

#[test]
fn psubb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RDX, 999422727, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 248, 178, 7, 251, 145, 59], OperandSize::Qword)
}

