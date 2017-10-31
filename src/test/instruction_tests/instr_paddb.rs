use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 252, 234], OperandSize::Dword)
}

#[test]
fn paddb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 252, 12, 90], OperandSize::Dword)
}

#[test]
fn paddb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 252, 241], OperandSize::Qword)
}

#[test]
fn paddb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 252, 28, 150], OperandSize::Qword)
}

#[test]
fn paddb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 252, 202], OperandSize::Dword)
}

#[test]
fn paddb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Four, 859634605, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 252, 156, 138, 173, 251, 60, 51], OperandSize::Dword)
}

#[test]
fn paddb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 252, 200], OperandSize::Qword)
}

#[test]
fn paddb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RBX, RBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 252, 44, 91], OperandSize::Qword)
}

