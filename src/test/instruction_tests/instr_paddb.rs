use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 252, 220], OperandSize::Dword)
}

#[test]
fn paddb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 1269082833, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 252, 140, 241, 209, 170, 164, 75], OperandSize::Dword)
}

#[test]
fn paddb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 252, 193], OperandSize::Qword)
}

#[test]
fn paddb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(RSI, 1976332885, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 252, 190, 85, 114, 204, 117], OperandSize::Qword)
}

#[test]
fn paddb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 252, 250], OperandSize::Dword)
}

#[test]
fn paddb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EAX, ECX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 252, 52, 136], OperandSize::Dword)
}

#[test]
fn paddb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 252, 196], OperandSize::Qword)
}

#[test]
fn paddb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 252, 36, 183], OperandSize::Qword)
}

