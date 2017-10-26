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
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 0], OperandSize::Word)
}

#[test]
fn seta_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 195], OperandSize::Dword)
}

#[test]
fn seta_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(IndirectScaledIndexed(EDI, ESI, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 4, 183], OperandSize::Dword)
}

#[test]
fn seta_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 193], OperandSize::Qword)
}

#[test]
fn seta_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(IndirectScaledDisplaced(RAX, Two, 914633274, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 4, 69, 58, 50, 132, 54], OperandSize::Qword)
}

#[test]
fn seta_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 194], OperandSize::Qword)
}

#[test]
fn seta_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 1875021325, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 132, 201, 13, 142, 194, 111], OperandSize::Qword)
}

