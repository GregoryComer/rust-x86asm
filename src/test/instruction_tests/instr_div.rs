use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn div_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 242], OperandSize::Word)
}

#[test]
fn div_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 27123, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 178, 243, 105], OperandSize::Word)
}

#[test]
fn div_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 241], OperandSize::Dword)
}

#[test]
fn div_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectDisplaced(EDI, 1465425754, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 183, 90, 159, 88, 87], OperandSize::Dword)
}

#[test]
fn div_5() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 243], OperandSize::Qword)
}

#[test]
fn div_6() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 52, 176], OperandSize::Qword)
}

#[test]
fn div_7() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 242], OperandSize::Qword)
}

#[test]
fn div_8() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Indirect(RDI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 55], OperandSize::Qword)
}

#[test]
fn div_9() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(CX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 241], OperandSize::Word)
}

#[test]
fn div_10() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 50], OperandSize::Word)
}

#[test]
fn div_11() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 245], OperandSize::Dword)
}

#[test]
fn div_12() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectScaledIndexed(EAX, EAX, Four, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 52, 128], OperandSize::Dword)
}

#[test]
fn div_13() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 247], OperandSize::Qword)
}

#[test]
fn div_14() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectDisplaced(RDX, 650349835, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 178, 11, 141, 195, 38], OperandSize::Qword)
}

#[test]
fn div_15() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 246], OperandSize::Word)
}

#[test]
fn div_16() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 89, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 114, 89], OperandSize::Word)
}

#[test]
fn div_17() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(EDX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 242], OperandSize::Dword)
}

#[test]
fn div_18() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 1147511478, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 180, 178, 182, 162, 101, 68], OperandSize::Dword)
}

#[test]
fn div_19() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 241], OperandSize::Qword)
}

#[test]
fn div_20() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 48], OperandSize::Qword)
}

#[test]
fn div_21() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(RCX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 241], OperandSize::Qword)
}

#[test]
fn div_22() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Eight, 154893722, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 180, 242, 154, 125, 59, 9], OperandSize::Qword)
}

