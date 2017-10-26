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
    run_test(&Instruction { mnemonic: Mnemonic::SETNLE, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 174, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 130, 174, 0], OperandSize::Word)
}

#[test]
fn setnle_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNLE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 193], OperandSize::Dword)
}

#[test]
fn setnle_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNLE, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Eight, 1743897090, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 132, 248, 2, 194, 241, 103], OperandSize::Dword)
}

#[test]
fn setnle_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNLE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 193], OperandSize::Qword)
}

#[test]
fn setnle_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNLE, operand1: Some(IndirectScaledDisplaced(RDX, Four, 775504372, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 4, 149, 244, 65, 57, 46], OperandSize::Qword)
}

