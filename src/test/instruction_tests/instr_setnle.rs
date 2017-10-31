use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setnle_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNLE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 194], OperandSize::Word)
}

#[test]
fn setnle_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNLE, operand1: Some(IndirectDisplaced(BX, 49, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 71, 49], OperandSize::Word)
}

#[test]
fn setnle_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNLE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 193], OperandSize::Dword)
}

#[test]
fn setnle_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNLE, operand1: Some(IndirectScaledDisplaced(EDX, Two, 993497395, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 4, 85, 51, 145, 55, 59], OperandSize::Dword)
}

#[test]
fn setnle_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNLE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 195], OperandSize::Qword)
}

#[test]
fn setnle_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNLE, operand1: Some(IndirectDisplaced(RDX, 452454387, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 130, 243, 231, 247, 26], OperandSize::Qword)
}

