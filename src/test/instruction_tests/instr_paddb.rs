use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 252, 199], OperandSize::Dword)
}

#[test]
fn paddb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 1060667755, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 252, 28, 149, 107, 129, 56, 63], OperandSize::Dword)
}

#[test]
fn paddb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 252, 216], OperandSize::Qword)
}

#[test]
fn paddb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(MM6)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 252, 48], OperandSize::Qword)
}

#[test]
fn paddb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 252, 203], OperandSize::Dword)
}

#[test]
fn paddb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Four, 1539471648, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 252, 164, 145, 32, 121, 194, 91], OperandSize::Dword)
}

#[test]
fn paddb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 252, 223], OperandSize::Qword)
}

#[test]
fn paddb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 252, 18], OperandSize::Qword)
}

