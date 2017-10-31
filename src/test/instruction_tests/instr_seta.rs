use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn seta_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 193], OperandSize::Word)
}

#[test]
fn seta_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 4366, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 128, 14, 17], OperandSize::Word)
}

#[test]
fn seta_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 195], OperandSize::Dword)
}

#[test]
fn seta_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(IndirectScaledDisplaced(ESI, Four, 405542323, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 4, 181, 179, 21, 44, 24], OperandSize::Dword)
}

#[test]
fn seta_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 195], OperandSize::Qword)
}

#[test]
fn seta_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(IndirectDisplaced(RCX, 962797913, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 129, 89, 33, 99, 57], OperandSize::Qword)
}

#[test]
fn seta_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 193], OperandSize::Qword)
}

#[test]
fn seta_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(IndirectScaledDisplaced(RSI, Two, 222216488, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 4, 117, 40, 193, 62, 13], OperandSize::Qword)
}

