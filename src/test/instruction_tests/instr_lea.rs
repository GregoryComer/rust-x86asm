use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lea_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[141, 40], OperandSize::Word)
}

#[test]
fn lea_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Two, 1257204882, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 141, 148, 71, 146, 108, 239, 74], OperandSize::Dword)
}

#[test]
fn lea_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 815592776, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 141, 20, 245, 72, 245, 156, 48], OperandSize::Qword)
}

#[test]
fn lea_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(BX, 13, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 141, 127, 13], OperandSize::Word)
}

#[test]
fn lea_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Four, 1903740322, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[141, 156, 142, 162, 197, 120, 113], OperandSize::Dword)
}

#[test]
fn lea_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexed(RDI, RDI, Eight, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[141, 20, 255], OperandSize::Qword)
}

#[test]
fn lea_7() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(RSI)), operand2: Some(Indirect(RDI, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 141, 55], OperandSize::Qword)
}

