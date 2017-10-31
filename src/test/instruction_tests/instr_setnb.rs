use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setnb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 194], OperandSize::Word)
}

#[test]
fn setnb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 6710, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 129, 54, 26], OperandSize::Word)
}

#[test]
fn setnb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 194], OperandSize::Dword)
}

#[test]
fn setnb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(IndirectScaledDisplaced(EAX, Eight, 208290220, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 4, 197, 172, 65, 106, 12], OperandSize::Dword)
}

#[test]
fn setnb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 195], OperandSize::Qword)
}

#[test]
fn setnb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 4, 208], OperandSize::Qword)
}

#[test]
fn setnb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 195], OperandSize::Qword)
}

#[test]
fn setnb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Two, 514002444, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 132, 72, 12, 14, 163, 30], OperandSize::Qword)
}

