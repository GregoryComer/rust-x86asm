use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ins_1() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 1704, Some(OperandSize::Byte), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[108], OperandSize::Word)
}

#[test]
fn ins_2() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(Indirect(EBX, Some(OperandSize::Byte), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[108], OperandSize::Dword)
}

#[test]
fn ins_3() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(IndirectScaledIndexed(RAX, RSI, Two, Some(OperandSize::Byte), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[108], OperandSize::Qword)
}

#[test]
fn ins_4() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(IndirectDisplaced(SI, 26633, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[109], OperandSize::Word)
}

#[test]
fn ins_5() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(IndirectScaledDisplaced(EAX, Four, 424500646, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 109], OperandSize::Dword)
}

#[test]
fn ins_6() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 227110434, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 109], OperandSize::Qword)
}

#[test]
fn ins_7() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 25086, Some(OperandSize::Dword), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 109], OperandSize::Word)
}

#[test]
fn ins_8() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(IndirectScaledDisplaced(EDI, Eight, 2071876365, Some(OperandSize::Dword), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[109], OperandSize::Dword)
}

#[test]
fn ins_9() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[109], OperandSize::Qword)
}

