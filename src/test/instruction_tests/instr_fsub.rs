use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fsub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 7918, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 162, 238, 30], OperandSize::Word)
}

#[test]
fn fsub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(IndirectScaledIndexed(EDI, ECX, Eight, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 36, 207], OperandSize::Dword)
}

#[test]
fn fsub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 36, 135], OperandSize::Qword)
}

#[test]
fn fsub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Direct(ST)), operand2: Some(Direct(ST7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 231], OperandSize::Word)
}

#[test]
fn fsub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Direct(ST)), operand2: Some(Direct(ST3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 227], OperandSize::Dword)
}

#[test]
fn fsub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Direct(ST)), operand2: Some(Direct(ST5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 229], OperandSize::Qword)
}

#[test]
fn fsub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(IndirectDisplaced(BP, 25283, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 166, 195, 98], OperandSize::Word)
}

#[test]
fn fsub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 36, 83], OperandSize::Dword)
}

#[test]
fn fsub_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 38], OperandSize::Qword)
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
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Direct(ST2)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 234], OperandSize::Qword)
}

