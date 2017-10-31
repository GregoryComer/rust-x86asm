use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ins_1() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 4, Some(OperandSize::Byte), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[108], OperandSize::Word)
}

#[test]
fn ins_2() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(IndirectDisplaced(ESI, 2106816985, Some(OperandSize::Byte), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[108], OperandSize::Dword)
}

#[test]
fn ins_3() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(IndirectScaledIndexed(RSI, RSI, Two, Some(OperandSize::Byte), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[108], OperandSize::Qword)
}

#[test]
fn ins_4() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[109], OperandSize::Word)
}

#[test]
fn ins_5() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(IndirectDisplaced(EDI, 405950304, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 109], OperandSize::Dword)
}

#[test]
fn ins_6() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 1014973581, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 109], OperandSize::Qword)
}

#[test]
fn ins_7() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 67, Some(OperandSize::Dword), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 109], OperandSize::Word)
}

#[test]
fn ins_8() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(IndirectScaledIndexed(ECX, EBX, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[109], OperandSize::Dword)
}

#[test]
fn ins_9() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 253268062, Some(OperandSize::Dword), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[109], OperandSize::Qword)
}

