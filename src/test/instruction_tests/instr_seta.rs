use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn seta_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 194], OperandSize::Word)
}

#[test]
fn seta_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(IndirectDisplaced(SI, 6, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 68, 6], OperandSize::Word)
}

#[test]
fn seta_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 194], OperandSize::Dword)
}

#[test]
fn seta_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 4, 190], OperandSize::Dword)
}

#[test]
fn seta_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 195], OperandSize::Qword)
}

#[test]
fn seta_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Eight, 1096679191, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 132, 248, 23, 255, 93, 65], OperandSize::Qword)
}

#[test]
fn seta_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 193], OperandSize::Qword)
}

#[test]
fn seta_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(IndirectScaledDisplaced(RDI, Eight, 1804826049, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 4, 253, 193, 117, 147, 107], OperandSize::Qword)
}

