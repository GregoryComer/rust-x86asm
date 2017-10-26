use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn mul_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 226], OperandSize::Word)
}

#[test]
fn mul_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 34], OperandSize::Word)
}

#[test]
fn mul_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 226], OperandSize::Dword)
}

#[test]
fn mul_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 1375119463, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 164, 119, 103, 168, 246, 81], OperandSize::Dword)
}

#[test]
fn mul_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 225], OperandSize::Qword)
}

#[test]
fn mul_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectDisplaced(RDI, 879126340, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 167, 68, 103, 102, 52], OperandSize::Qword)
}

#[test]
fn mul_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 226], OperandSize::Qword)
}

#[test]
fn mul_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledIndexed(RDI, RDX, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 36, 215], OperandSize::Qword)
}

#[test]
fn mul_9() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(CX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 225], OperandSize::Word)
}

#[test]
fn mul_10() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 34], OperandSize::Word)
}

#[test]
fn mul_11() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 230], OperandSize::Dword)
}

#[test]
fn mul_12() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 32], OperandSize::Dword)
}

#[test]
fn mul_13() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 226], OperandSize::Qword)
}

#[test]
fn mul_14() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledIndexed(RCX, RDX, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 36, 209], OperandSize::Qword)
}

#[test]
fn mul_15() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(EBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 229], OperandSize::Word)
}

#[test]
fn mul_16() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 33], OperandSize::Word)
}

#[test]
fn mul_17() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 230], OperandSize::Dword)
}

#[test]
fn mul_18() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 39], OperandSize::Dword)
}

#[test]
fn mul_19() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(EBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 227], OperandSize::Qword)
}

#[test]
fn mul_20() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 1204680245, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 164, 216, 53, 246, 205, 71], OperandSize::Qword)
}

#[test]
fn mul_21() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(RDX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 226], OperandSize::Qword)
}

#[test]
fn mul_22() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledIndexed(RDI, RCX, Two, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 36, 79], OperandSize::Qword)
}

