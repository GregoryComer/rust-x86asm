use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setna_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 193], OperandSize::Word)
}

#[test]
fn setna_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(IndirectDisplaced(DI, 3193, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 133, 121, 12], OperandSize::Word)
}

#[test]
fn setna_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 194], OperandSize::Dword)
}

#[test]
fn setna_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(IndirectScaledDisplaced(EDX, Eight, 1738680817, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 4, 213, 241, 41, 162, 103], OperandSize::Dword)
}

#[test]
fn setna_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 193], OperandSize::Qword)
}

#[test]
fn setna_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(IndirectDisplaced(RAX, 161870182, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 128, 102, 241, 165, 9], OperandSize::Qword)
}

#[test]
fn setna_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 195], OperandSize::Qword)
}

#[test]
fn setna_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(Indirect(RDI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 7], OperandSize::Qword)
}

