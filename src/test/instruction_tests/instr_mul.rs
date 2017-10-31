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
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 34], OperandSize::Word)
}

#[test]
fn mul_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 227], OperandSize::Dword)
}

#[test]
fn mul_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledIndexed(ECX, ESI, Two, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 36, 113], OperandSize::Dword)
}

#[test]
fn mul_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 225], OperandSize::Qword)
}

#[test]
fn mul_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 33], OperandSize::Qword)
}

#[test]
fn mul_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 225], OperandSize::Qword)
}

#[test]
fn mul_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledDisplaced(RCX, Two, 947517618, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 36, 77, 178, 248, 121, 56], OperandSize::Qword)
}

#[test]
fn mul_9() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 226], OperandSize::Word)
}

#[test]
fn mul_10() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 105, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 96, 105], OperandSize::Word)
}

#[test]
fn mul_11() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 230], OperandSize::Dword)
}

#[test]
fn mul_12() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledDisplaced(EDI, Four, 15025319, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 36, 189, 167, 68, 229, 0], OperandSize::Dword)
}

#[test]
fn mul_13() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 229], OperandSize::Qword)
}

#[test]
fn mul_14() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 36, 150], OperandSize::Qword)
}

#[test]
fn mul_15() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 230], OperandSize::Word)
}

#[test]
fn mul_16() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectDisplaced(SI, 26063, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 164, 207, 101], OperandSize::Word)
}

#[test]
fn mul_17() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(EBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 227], OperandSize::Dword)
}

#[test]
fn mul_18() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectDisplaced(ECX, 2046594595, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 161, 35, 142, 252, 121], OperandSize::Dword)
}

#[test]
fn mul_19() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(ESP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 228], OperandSize::Qword)
}

#[test]
fn mul_20() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 444552112, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 164, 209, 176, 83, 127, 26], OperandSize::Qword)
}

#[test]
fn mul_21() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(RSI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 230], OperandSize::Qword)
}

#[test]
fn mul_22() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledDisplaced(RDI, Four, 2092894840, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 36, 189, 120, 10, 191, 124], OperandSize::Qword)
}

