use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn div_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 241], OperandSize::Word)
}

#[test]
fn div_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 86, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 113, 86], OperandSize::Word)
}

#[test]
fn div_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 243], OperandSize::Dword)
}

#[test]
fn div_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectScaledIndexed(EDI, EBX, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 52, 159], OperandSize::Dword)
}

#[test]
fn div_5() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 242], OperandSize::Qword)
}

#[test]
fn div_6() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectScaledDisplaced(RAX, Two, 796591451, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 52, 69, 91, 5, 123, 47], OperandSize::Qword)
}

#[test]
fn div_7() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 242], OperandSize::Qword)
}

#[test]
fn div_8() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 383480372, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 180, 146, 52, 114, 219, 22], OperandSize::Qword)
}

#[test]
fn div_9() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 242], OperandSize::Word)
}

#[test]
fn div_10() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectDisplaced(BX, 18388, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 183, 212, 71], OperandSize::Word)
}

#[test]
fn div_11() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 245], OperandSize::Dword)
}

#[test]
fn div_12() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 1709300924, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 180, 80, 188, 220, 225, 101], OperandSize::Dword)
}

#[test]
fn div_13() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 244], OperandSize::Qword)
}

#[test]
fn div_14() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectScaledIndexed(RSI, RBX, Four, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 52, 158], OperandSize::Qword)
}

#[test]
fn div_15() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(ESP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 244], OperandSize::Word)
}

#[test]
fn div_16() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 52], OperandSize::Word)
}

#[test]
fn div_17() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(EDI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 247], OperandSize::Dword)
}

#[test]
fn div_18() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 49], OperandSize::Dword)
}

#[test]
fn div_19() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(EBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 245], OperandSize::Qword)
}

#[test]
fn div_20() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Eight, 1633226544, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 180, 215, 48, 15, 89, 97], OperandSize::Qword)
}

#[test]
fn div_21() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(RSP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 244], OperandSize::Qword)
}

#[test]
fn div_22() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 48], OperandSize::Qword)
}

