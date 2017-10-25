use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ins_1() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 146, Some(OperandSize::Byte), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[108], OperandSize::Word)
}

#[test]
fn ins_2() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(IndirectDisplaced(ECX, 1822113522, Some(OperandSize::Byte), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[108], OperandSize::Dword)
}

#[test]
fn ins_3() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(IndirectDisplaced(RAX, 31926513, Some(OperandSize::Byte), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[108], OperandSize::Qword)
}

#[test]
fn ins_4() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 161, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[109], OperandSize::Word)
}

#[test]
fn ins_5() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 109], OperandSize::Dword)
}

#[test]
fn ins_6() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 109], OperandSize::Qword)
}

#[test]
fn ins_7() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(IndirectDisplaced(SI, 7732, Some(OperandSize::Dword), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 109], OperandSize::Word)
}

#[test]
fn ins_8() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[109], OperandSize::Dword)
}

#[test]
fn ins_9() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Dword), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[109], OperandSize::Qword)
}

