use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setnbe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 195], OperandSize::Word)
}

#[test]
fn setnbe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 27666, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 129, 18, 108], OperandSize::Word)
}

#[test]
fn setnbe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 194], OperandSize::Dword)
}

#[test]
fn setnbe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 4, 88], OperandSize::Dword)
}

#[test]
fn setnbe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 194], OperandSize::Qword)
}

#[test]
fn setnbe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 668657101, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 132, 127, 205, 229, 218, 39], OperandSize::Qword)
}

#[test]
fn setnbe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 195], OperandSize::Qword)
}

#[test]
fn setnbe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(IndirectDisplaced(RDI, 896438042, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 135, 26, 143, 110, 53], OperandSize::Qword)
}

