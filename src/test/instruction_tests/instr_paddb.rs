use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 252, 195], OperandSize::Dword)
}

#[test]
fn paddb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(MM2)), operand2: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 252, 19], OperandSize::Dword)
}

#[test]
fn paddb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 252, 205], OperandSize::Qword)
}

#[test]
fn paddb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(RSI, 577807318, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 252, 150, 214, 163, 112, 34], OperandSize::Qword)
}

#[test]
fn paddb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 252, 246], OperandSize::Dword)
}

#[test]
fn paddb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Eight, 1988930077, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 252, 132, 249, 29, 170, 140, 118], OperandSize::Dword)
}

#[test]
fn paddb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 252, 193], OperandSize::Qword)
}

#[test]
fn paddb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RCX, 636318541, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 252, 177, 77, 115, 237, 37], OperandSize::Qword)
}

