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
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 16603, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 162, 219, 64], OperandSize::Word)
}

#[test]
fn mul_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 227], OperandSize::Dword)
}

#[test]
fn mul_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 36, 193], OperandSize::Dword)
}

#[test]
fn mul_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 226], OperandSize::Qword)
}

#[test]
fn mul_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 1246321763, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 164, 190, 99, 92, 73, 74], OperandSize::Qword)
}

#[test]
fn mul_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 227], OperandSize::Qword)
}

#[test]
fn mul_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 1992078482, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 36, 221, 146, 180, 188, 118], OperandSize::Qword)
}

#[test]
fn mul_9() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 227], OperandSize::Word)
}

#[test]
fn mul_10() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 247, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 161, 247, 0], OperandSize::Word)
}

#[test]
fn mul_11() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 227], OperandSize::Dword)
}

#[test]
fn mul_12() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectDisplaced(EBX, 1594578534, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 163, 102, 86, 11, 95], OperandSize::Dword)
}

#[test]
fn mul_13() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 231], OperandSize::Qword)
}

#[test]
fn mul_14() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectDisplaced(RDI, 2112082118, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 167, 198, 208, 227, 125], OperandSize::Qword)
}

#[test]
fn mul_15() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 230], OperandSize::Word)
}

#[test]
fn mul_16() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectDisplaced(BX, 141, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 167, 141, 0], OperandSize::Word)
}

#[test]
fn mul_17() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(ESP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 228], OperandSize::Dword)
}

#[test]
fn mul_18() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectDisplaced(EAX, 302024234, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 160, 42, 134, 0, 18], OperandSize::Dword)
}

#[test]
fn mul_19() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(ESP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 228], OperandSize::Qword)
}

#[test]
fn mul_20() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectDisplaced(RSI, 1697236924, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 166, 188, 199, 41, 101], OperandSize::Qword)
}

#[test]
fn mul_21() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(RBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 229], OperandSize::Qword)
}

#[test]
fn mul_22() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectDisplaced(RBX, 385896085, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 163, 149, 78, 0, 23], OperandSize::Qword)
}

