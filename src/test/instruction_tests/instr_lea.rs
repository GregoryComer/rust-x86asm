use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lea_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(SI, 24784, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[141, 148, 208, 96], OperandSize::Word)
}

#[test]
fn lea_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(SP)), operand2: Some(Indirect(EAX, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 141, 32], OperandSize::Dword)
}

#[test]
fn lea_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 1026869551, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 141, 52, 189, 47, 201, 52, 61], OperandSize::Qword)
}

#[test]
fn lea_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 141, 50], OperandSize::Word)
}

#[test]
fn lea_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 1470238587, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[141, 28, 221, 123, 15, 162, 87], OperandSize::Dword)
}

#[test]
fn lea_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(EBX)), operand2: Some(Indirect(RDX, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[141, 26], OperandSize::Qword)
}

#[test]
fn lea_7() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexed(RBX, RAX, Two, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 141, 28, 67], OperandSize::Qword)
}

