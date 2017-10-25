use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 248, 209], OperandSize::Dword)
}

#[test]
fn psubb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Eight, 1087524119, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 248, 148, 211, 23, 77, 210, 64], OperandSize::Dword)
}

#[test]
fn psubb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 248, 216], OperandSize::Qword)
}

#[test]
fn psubb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 466684486, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 248, 60, 245, 70, 10, 209, 27], OperandSize::Qword)
}

#[test]
fn psubb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 248, 192], OperandSize::Dword)
}

#[test]
fn psubb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(ECX, 274743137, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 248, 129, 97, 63, 96, 16], OperandSize::Dword)
}

#[test]
fn psubb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 248, 249], OperandSize::Qword)
}

#[test]
fn psubb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 248, 44, 249], OperandSize::Qword)
}

