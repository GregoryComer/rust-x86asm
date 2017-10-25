use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setnge_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 195], OperandSize::Word)
}

#[test]
fn setnge_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 3], OperandSize::Word)
}

#[test]
fn setnge_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 194], OperandSize::Dword)
}

#[test]
fn setnge_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(IndirectScaledDisplaced(EDX, Eight, 693023406, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 4, 213, 174, 178, 78, 41], OperandSize::Dword)
}

#[test]
fn setnge_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 193], OperandSize::Qword)
}

#[test]
fn setnge_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(IndirectDisplaced(RSI, 1228786038, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 134, 118, 201, 61, 73], OperandSize::Qword)
}

#[test]
fn setnge_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 195], OperandSize::Qword)
}

#[test]
fn setnge_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 1], OperandSize::Qword)
}

