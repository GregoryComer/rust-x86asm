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
    run_test(&Instruction { mnemonic: Mnemonic::SETNLE, operand1: Some(IndirectDisplaced(BP, 11634, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 134, 114, 45], OperandSize::Word)
}

#[test]
fn setnle_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNLE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 193], OperandSize::Dword)
}

#[test]
fn setnle_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNLE, operand1: Some(Indirect(EAX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 0], OperandSize::Dword)
}

#[test]
fn setnle_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNLE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 193], OperandSize::Qword)
}

#[test]
fn setnle_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNLE, operand1: Some(IndirectDisplaced(RDI, 1400149463, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 135, 215, 149, 116, 83], OperandSize::Qword)
}

