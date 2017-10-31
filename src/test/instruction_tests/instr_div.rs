use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn div_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 243], OperandSize::Word)
}

#[test]
fn div_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectDisplaced(BP, 17109, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 182, 213, 66], OperandSize::Word)
}

#[test]
fn div_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 241], OperandSize::Dword)
}

#[test]
fn div_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectDisplaced(ECX, 25338854, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 177, 230, 163, 130, 1], OperandSize::Dword)
}

#[test]
fn div_5() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 243], OperandSize::Qword)
}

#[test]
fn div_6() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectDisplaced(RDI, 490306252, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 183, 204, 122, 57, 29], OperandSize::Qword)
}

#[test]
fn div_7() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 242], OperandSize::Qword)
}

#[test]
fn div_8() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectDisplaced(RSI, 23254066, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 182, 50, 212, 98, 1], OperandSize::Qword)
}

#[test]
fn div_9() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 246], OperandSize::Word)
}

#[test]
fn div_10() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectDisplaced(DI, 27979, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 181, 75, 109], OperandSize::Word)
}

#[test]
fn div_11() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 245], OperandSize::Dword)
}

#[test]
fn div_12() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectScaledDisplaced(EDX, Four, 1944901219, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 52, 149, 99, 214, 236, 115], OperandSize::Dword)
}

#[test]
fn div_13() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 244], OperandSize::Qword)
}

#[test]
fn div_14() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 2078000800, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 52, 197, 160, 198, 219, 123], OperandSize::Qword)
}

#[test]
fn div_15() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(EBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 243], OperandSize::Word)
}

#[test]
fn div_16() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 4372, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 178, 20, 17], OperandSize::Word)
}

#[test]
fn div_17() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(EDX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 242], OperandSize::Dword)
}

#[test]
fn div_18() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectScaledDisplaced(EDX, Four, 417288955, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 52, 149, 251, 82, 223, 24], OperandSize::Dword)
}

#[test]
fn div_19() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(EBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 245], OperandSize::Qword)
}

#[test]
fn div_20() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectDisplaced(RDX, 1583400644, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 178, 196, 198, 96, 94], OperandSize::Qword)
}

#[test]
fn div_21() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(RSP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 244], OperandSize::Qword)
}

#[test]
fn div_22() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Four, 1689275708, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 180, 155, 60, 77, 176, 100], OperandSize::Qword)
}

