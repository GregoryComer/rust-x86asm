use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn mul_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 225], OperandSize::Word)
}

#[test]
fn mul_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 112, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 99, 112], OperandSize::Word)
}

#[test]
fn mul_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 227], OperandSize::Dword)
}

#[test]
fn mul_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledIndexed(EDX, EDX, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 36, 146], OperandSize::Dword)
}

#[test]
fn mul_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 227], OperandSize::Qword)
}

#[test]
fn mul_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Indirect(RDI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 39], OperandSize::Qword)
}

#[test]
fn mul_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 225], OperandSize::Qword)
}

#[test]
fn mul_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledDisplaced(RSI, Two, 604255777, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 36, 117, 33, 54, 4, 36], OperandSize::Qword)
}

#[test]
fn mul_9() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 227], OperandSize::Word)
}

#[test]
fn mul_10() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 33], OperandSize::Word)
}

#[test]
fn mul_11() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 230], OperandSize::Dword)
}

#[test]
fn mul_12() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 38], OperandSize::Dword)
}

#[test]
fn mul_13() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 229], OperandSize::Qword)
}

#[test]
fn mul_14() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 1626855081, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 164, 146, 169, 214, 247, 96], OperandSize::Qword)
}

#[test]
fn mul_15() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 230], OperandSize::Word)
}

#[test]
fn mul_16() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Memory(32011, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 38, 11, 125], OperandSize::Word)
}

#[test]
fn mul_17() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(EDX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 226], OperandSize::Dword)
}

#[test]
fn mul_18() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledDisplaced(ECX, Two, 1031011537, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 36, 77, 209, 252, 115, 61], OperandSize::Dword)
}

#[test]
fn mul_19() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(EBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 227], OperandSize::Qword)
}

#[test]
fn mul_20() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 1471912909, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 36, 245, 205, 155, 187, 87], OperandSize::Qword)
}

#[test]
fn mul_21() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(RDI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 231], OperandSize::Qword)
}

#[test]
fn mul_22() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 1854852933, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 164, 152, 69, 207, 142, 110], OperandSize::Qword)
}

