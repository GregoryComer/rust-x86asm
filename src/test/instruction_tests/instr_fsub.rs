use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fsub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(IndirectDisplaced(BX, 204, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 167, 204, 0], OperandSize::Word)
}

#[test]
fn fsub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(IndirectScaledDisplaced(ESI, Four, 309217085, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 36, 181, 61, 71, 110, 18], OperandSize::Dword)
}

#[test]
fn fsub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 32], OperandSize::Qword)
}

#[test]
fn fsub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Direct(ST)), operand2: Some(Direct(ST5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 229], OperandSize::Word)
}

#[test]
fn fsub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Direct(ST)), operand2: Some(Direct(ST7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 231], OperandSize::Dword)
}

#[test]
fn fsub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Direct(ST)), operand2: Some(Direct(ST3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 227], OperandSize::Qword)
}

#[test]
fn fsub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 23671, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 161, 119, 92], OperandSize::Word)
}

#[test]
fn fsub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 36, 158], OperandSize::Dword)
}

#[test]
fn fsub_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(IndirectDisplaced(RSI, 1861984418, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 166, 162, 160, 251, 110], OperandSize::Qword)
}

#[test]
fn fsub_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Direct(ST1)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 233], OperandSize::Word)
}

#[test]
fn fsub_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Direct(ST3)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 235], OperandSize::Dword)
}

#[test]
fn fsub_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Direct(ST5)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 237], OperandSize::Qword)
}

