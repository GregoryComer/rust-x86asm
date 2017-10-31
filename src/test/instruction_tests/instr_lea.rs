use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lea_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[141, 26], OperandSize::Word)
}

#[test]
fn lea_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(DI)), operand2: Some(Indirect(ECX, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 141, 57], OperandSize::Dword)
}

#[test]
fn lea_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1225171050, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 141, 44, 205, 106, 160, 6, 73], OperandSize::Qword)
}

#[test]
fn lea_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(SI, 6539, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 141, 188, 139, 25], OperandSize::Word)
}

#[test]
fn lea_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 1996382547, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[141, 28, 197, 83, 97, 254, 118], OperandSize::Dword)
}

#[test]
fn lea_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(ESI)), operand2: Some(Indirect(RDI, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[141, 55], OperandSize::Qword)
}

#[test]
fn lea_7() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 2021483233, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 141, 148, 83, 225, 98, 125, 120], OperandSize::Qword)
}

