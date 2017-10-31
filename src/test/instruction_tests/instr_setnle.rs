use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setnle_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNLE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 195], OperandSize::Word)
}

#[test]
fn setnle_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNLE, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 13369, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 129, 57, 52], OperandSize::Word)
}

#[test]
fn setnle_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNLE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 193], OperandSize::Dword)
}

#[test]
fn setnle_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNLE, operand1: Some(IndirectScaledDisplaced(ESI, Four, 1067663084, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 4, 181, 236, 62, 163, 63], OperandSize::Dword)
}

#[test]
fn setnle_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNLE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 193], OperandSize::Qword)
}

#[test]
fn setnle_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNLE, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 1845974285, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 4, 213, 13, 85, 7, 110], OperandSize::Qword)
}

