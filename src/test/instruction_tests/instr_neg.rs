use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn neg_1() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 218], OperandSize::Word)
}

#[test]
fn neg_2() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 27], OperandSize::Word)
}

#[test]
fn neg_3() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 219], OperandSize::Dword)
}

#[test]
fn neg_4() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Indirect(EDI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 31], OperandSize::Dword)
}

#[test]
fn neg_5() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 219], OperandSize::Qword)
}

#[test]
fn neg_6() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 27], OperandSize::Qword)
}

#[test]
fn neg_7() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 217], OperandSize::Qword)
}

#[test]
fn neg_8() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectDisplaced(RSI, 185448428, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 158, 236, 183, 13, 11], OperandSize::Qword)
}

#[test]
fn neg_9() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 220], OperandSize::Word)
}

#[test]
fn neg_10() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 25], OperandSize::Word)
}

#[test]
fn neg_11() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 222], OperandSize::Dword)
}

#[test]
fn neg_12() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledDisplaced(EBX, Two, 826602305, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 28, 93, 65, 243, 68, 49], OperandSize::Dword)
}

#[test]
fn neg_13() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 222], OperandSize::Qword)
}

#[test]
fn neg_14() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Indirect(RAX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 24], OperandSize::Qword)
}

#[test]
fn neg_15() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 222], OperandSize::Word)
}

#[test]
fn neg_16() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 47, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 91, 47], OperandSize::Word)
}

#[test]
fn neg_17() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(EDX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 218], OperandSize::Dword)
}

#[test]
fn neg_18() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 31], OperandSize::Dword)
}

#[test]
fn neg_19() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(EDI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 223], OperandSize::Qword)
}

#[test]
fn neg_20() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 28, 202], OperandSize::Qword)
}

#[test]
fn neg_21() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(RSI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 222], OperandSize::Qword)
}

#[test]
fn neg_22() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 28, 66], OperandSize::Qword)
}

